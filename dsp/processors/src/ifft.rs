use std::thread;
use std::sync::{Arc, Mutex};
use std::f64::consts::PI;
use num_complex::Complex;

use bb_processor::*;

fn perform_ifft(pipe: &DataPipe, period: i64){
    println!("Performing ifft...");
    let i_vec: &mut Vec<Complex<f64>> = &mut pipe.input.data.lock().unwrap();
    let o_vec: &mut Vec<Complex<f64>> = &mut pipe.output.data.lock().unwrap();
    let buffer_len = i_vec.len();

    for i in 0..buffer_len {
        for j in 0..buffer_len {
            // i is for each f_i element
            // j is for going through each array element and weighting
            let trig_arg: f64 = (2.0 * PI * (i as f64) * (j as f64)) / period as f64;

            let trig_arg_sin: f64 = trig_arg.sin();
            let trig_arg_cos: f64 = trig_arg.cos();

            let real_product: f64 = (i_vec[j].re * trig_arg_cos) - (i_vec[j].im * trig_arg_sin);
            let imag_product: f64 = (i_vec[j].im * trig_arg_cos) + (i_vec[j].re * trig_arg_sin);

            o_vec[i].re += real_product;
            o_vec[i].im -= imag_product;
        }
    }
}

pub struct IFFT {
    buffer: BasicBufferProcessor
}

impl IFFT {
    pub fn new(buffer_len: usize, inputs: &Vec<HalfPipe>) -> IFFT {
        IFFT {
            buffer: BasicBufferProcessor::new(buffer_len, inputs, 0.0, "IFFT".to_owned())
        }
    }
}

#[allow(unused_variables)]
impl DspPathMember for IFFT {

    fn run(&mut self, clock: Arc<Mutex<i8>>, period: i64) -> () {

        // spawn thread for each channel and perform FFT
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
                    thread_handles.push(
                        thread::spawn(move || {
                            data_pipe_clone.wait_for_start();
                            perform_ifft(&data_pipe_clone, period);
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