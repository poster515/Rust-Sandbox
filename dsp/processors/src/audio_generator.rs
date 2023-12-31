
use std::thread;
use std::sync::{Arc, Mutex, Condvar};

use bb_processor::{BasicBufferProcessor, ReturnPair, DspBuffer, DspChannels, BufferTrait};
use generator::*;

pub trait DspPathMember {
    fn init(&mut self) -> ();
    fn run(&mut self, clk: Arc<Mutex<i8>>, period: i64) -> ();
    fn get_my_vec_ref(&self) -> DspChannels;
    fn get_cond_var_ptr(&self) -> ReturnPair;
}

pub struct AudioGenerator {
    buffer: BasicBufferProcessor
}

impl AudioGenerator {
    pub fn new(buffer_len: usize, pab: Arc<Mutex<bool>>, pcv: Arc<Condvar>, pb: DspChannels) -> AudioGenerator {

        AudioGenerator {
            buffer: BasicBufferProcessor::new(buffer_len, pab, pcv, pb, "AudioGenerator".to_owned())
        }
    }
}

#[allow(unused_variables)]
impl DspPathMember for AudioGenerator {
    fn init(&mut self) -> () {
        // just make new vectors for each channel
        let binding = self.buffer.get_my_vec_ref();
        let mut buffer = binding.lock().unwrap();
        for i in 0..4 {
            println!("AudioGenerator creating buffer #{}...", i);
            (*buffer).push(Arc::new(Mutex::new(Vec::new())));
        }
    }

    fn run(&mut self, clock: Arc<Mutex<i8>>, period: i64) -> () {

        // create new buffer for each channel
        self.init();

        // wait for start signal
        self.buffer.wait_for_start();

        // spin up threads for each channel and populate
        let func_vec: Vec<&(dyn Fn(DspBuffer
            , usize
            , Arc<Mutex<i8>>
            , i64) -> () + Send)> = vec![
                &generate_square_wave_data,
                &generate_sine_wave_data,
                &generate_triangle_wave_data,
                &generate_sawtooth_wave_data];

        let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();

        let binding: DspChannels = self.buffer.get_my_vec_ref();
        let buffer_len =  self.buffer.get_fill_length();

        for (index, func) in func_vec.into_iter().enumerate() {

            // println!("Running function number: {index:}")
            let buffer = binding.lock().unwrap();
            let v: Option<&DspBuffer> = (*buffer).get(index);
            match v {
                None => panic!("Could not locate vector!"),
                Some(vec) => {
                    let clk = Arc::clone(&clock);
                    let per = period;
                    let vec_clone = Arc::clone(&vec);

                    // TODO: actually call functions in func_vec
                    thread_handles.push(
                        thread::spawn(move || {
                            generate_square_wave_data(vec_clone, buffer_len, clk, per);
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

        self.buffer.end_of_processing();
    }

    fn get_my_vec_ref(&self) -> DspChannels {
        self.buffer.get_my_vec_ref()
    }

    fn get_cond_var_ptr(&self) -> ReturnPair {
        self.buffer.get_cond_var_ptr()
    }
}