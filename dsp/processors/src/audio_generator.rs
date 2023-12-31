
use std::thread;
use std::sync::{Arc, Mutex, Condvar};

use num_complex::Complex;


use bb_processor::{BasicBufferProcessor, ReturnPair, BufferTrait};
use generator::*;

pub trait DspPathMember {
    fn init(&mut self) -> ();
    fn run(&mut self, clk: Arc<Mutex<i8>>, period: i64) -> ();
    fn get_my_vec_ref(&self) -> Arc<Mutex<Vec<Vec<Complex<f64>>>>>;
    fn get_cond_var_ptr(&self) -> ReturnPair;
}

pub struct AudioGenerator {
    buffer: BasicBufferProcessor
}

impl AudioGenerator {
    pub fn new(buffer_len: usize, pab: Arc<Mutex<bool>>, pcv: Arc<Condvar>) -> AudioGenerator {

        AudioGenerator {
            buffer: BasicBufferProcessor::new(buffer_len, pab, pcv)
        }
    }
}

#[allow(unused_variables)]
impl DspPathMember for AudioGenerator {
    fn init(&mut self) -> () {
        // just make new vectors for each channel
        let binding = self.buffer.get_my_vec_ref();
        let mut buffer = binding.lock().unwrap();
        for _ in 0..3 {
            (*buffer).push(Vec::new());
        }

        assert_eq!(self.buffer.get_my_vec_ref().lock().unwrap().len(), 4);
    }

    fn run(&mut self, clock: Arc<Mutex<i8>>, period: i64) -> () {

        // create new buffer for each channel
        self.init();

        // wait for start signal
        self.buffer.wait_for_start();

        // spin up threads for each channel and populate
        let func_vec = vec![
                generate_square_wave_data,
                generate_sine_wave_data,
                generate_triangle_wave_data,
                generate_sawtooth_wave_data];

        let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();

        let binding = self.buffer.get_my_vec_ref();
        let buffer = binding.lock().unwrap();
        let buffer_len =  self.buffer.get_fill_length();

        for (index, func) in func_vec.iter().enumerate() {
            println!("Running function number: {index:}")
            // let v: Option<&mut Vec<Complex<f64>>> = (*buffer).get_mut(index);
            // match v {
            //     None => panic!("Could not locate vector!"),
            //     Some(vec) => {
            //         let clk = Arc::clone(&clock);
            //         let per = period;

            //         thread_handles.push(
            //             thread::spawn(move || {
            //                     func(vec, buffer_len, clk, per);
            //                 }
            //             )
            //         );
            //     }
            // };
        }

        // join all threads
        while thread_handles.len() > 0 {
            let cur_thread = thread_handles.remove(0); 
            cur_thread.join().unwrap();
        }

        self.buffer.end_of_processing();
    }

    fn get_my_vec_ref(&self) -> Arc<Mutex<Vec<Vec<Complex<f64>>>>> {
        self.buffer.get_my_vec_ref()
    }

    fn get_cond_var_ptr(&self) -> ReturnPair {
        self.buffer.get_cond_var_ptr()
    }
}