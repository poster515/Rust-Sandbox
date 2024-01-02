use std::thread;
use std::sync::{Arc, Mutex};
use num_complex::Complex;

use bb_processor::*;

fn filter(pipe: &DataPipe, filter: Arc<Mutex<Vec<f64>>>){
    println!("Filtering data...");
    let i_vec: &mut Vec<Complex<f64>> = &mut pipe.input.data.lock().unwrap();
    let o_vec: &mut Vec<Complex<f64>> = &mut pipe.output.data.lock().unwrap();
    
    let buffer_len = i_vec.len();

    for i in 0..buffer_len {
        // get lock each time so all threads can use this simultaneously
        let filter: &Vec<f64> = &filter.lock().unwrap();
        o_vec[i].re = i_vec[i].re * (*filter)[i];
        o_vec[i].im = i_vec[i].im * (*filter)[i];
    }
}

pub struct Filter {
    buffer: BasicBufferProcessor,
    filter: Arc<Mutex<Vec<f64>>>
}

impl Filter {
    pub fn new(buffer_len: usize, inputs: &Vec<HalfPipe>) -> Filter {
        let filter = Filter {
            buffer: BasicBufferProcessor::new(buffer_len, inputs, 0.0, "Filter".to_owned()),
            filter: Arc::new(Mutex::new(Vec::new()))
        };
        
        let mut binding = filter.filter.lock().unwrap();
        for i in 0..buffer_len {
            if i < buffer_len / 4 {
                (*binding).push(1.0);
            } else {
                (*binding).push(0.33);
            }
        }
        drop(binding);

        filter
    }
}

#[allow(unused_variables)]
impl DspPathMember for Filter {

    fn run(&mut self, clock: Arc<Mutex<i8>>, period: i64) -> () {

        // spawn thread for each channel and perform Filter
        let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();
        
        let num_channels = self.buffer.get_num_channels();
        let binding: DspChannels = self.buffer.get_channels();

        for index in 0..num_channels {
            let data_pipes = binding.lock().unwrap();
            let data_pipe = (*data_pipes).get(index);

            match data_pipe {
                None => panic!("Could not locate output vector!"),
                Some(dp) => {
                    let mut data_pipe_clone: DataPipe = dp.clone();
                    let filter_clone = Arc::clone(&self.filter);
                    thread_handles.push(
                        thread::spawn(move || {
                            data_pipe_clone.wait_for_start();
                            filter(&data_pipe_clone, filter_clone);
                            data_pipe_clone.end_of_processing();
                        })
                    );
                }
            };
        }

        // join all threads
        while thread_handles.len() > 0 {
            let cur_thread = thread_handles.remove(0); 
            cur_thread.join().unwrap();
        }

        self.buffer.write_to_file();
    }

    fn get_output_halfpipes(&self) -> Vec<HalfPipe> {
        self.buffer.get_output_halfpipes()
    }
}