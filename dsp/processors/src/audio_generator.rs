
use std::thread;
use std::sync::{Arc, Mutex};

mod generators;

use bb_processor::*;
use generators::*;


pub struct AudioGenerator {
    buffer: BasicBufferProcessor
}

impl AudioGenerator {
    pub fn new(buffer_len: usize, inputs: &Vec<HalfPipe>) -> AudioGenerator {
        AudioGenerator {
            buffer: BasicBufferProcessor::new(buffer_len, inputs, 0.0, "AudioGenerator".to_owned())
        }
    }
}

#[allow(unused_variables)]
impl DspPathMember for AudioGenerator {

    fn run(&mut self, clock: Arc<Mutex<i8>>, period: i64) -> () {

        // spin up threads for each channel and populate
        let func_vec: Vec<&(dyn Fn(DataBuffer
            , usize
            , Arc<Mutex<i8>>
            , i64) -> () + Send)> = vec![
                &generate_square_wave_data,
                &generate_sine_wave_data,
                &generate_triangle_wave_data,
                &generate_sawtooth_wave_data];

        let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();

        let binding: DspChannels = self.buffer.get_channels();
        let buffer_len =  self.buffer.get_fill_length();

        for (index, func) in func_vec.into_iter().enumerate() {

            // println!("Running function number: {index:}")
            let data_pipes = binding.lock().unwrap();
            let data_pipe = (*data_pipes).get(index);

            match data_pipe {
                None => panic!("Could not locate output vector!"),
                Some(dp) => {
                
                    let clk = Arc::clone(&clock);
                    let per = period;
                    let mut data_pipe_clone: DataPipe = dp.clone();

                    thread_handles.push(
                        thread::spawn(move || {
                            data_pipe_clone.wait_for_start();
                            generate_square_wave_data(data_pipe_clone.get_output_data_vec(), buffer_len, clk, per);
                            data_pipe_clone.end_of_processing();
                        })
                    );

                    // let func = func_vec.get(index).unwrap();
                    // let func_copy = &func;
                    // thread_handles.push(
                    //     thread::spawn(move || {
                    //             func(vec_clone, buffer_len, clk, per)
                    //         }
                    //     )
                    // );
                }
            };
        }

        // join all threads
        while thread_handles.len() > 0 {
            let cur_thread = thread_handles.remove(0); 
            cur_thread.join().unwrap();
        }
    }

    fn get_output_halfpipes(&self) -> Vec<HalfPipe> {
        self.buffer.get_output_halfpipes()
    }
}