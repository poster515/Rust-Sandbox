

use std::sync::{Arc, Mutex, Condvar};

use num_complex::Complex;

pub type ReturnPair = (Arc<Mutex<bool>>, Arc<Condvar>);
pub type DspBuffer = Arc<Mutex<Vec<Complex<f64>>>>;
pub type DspChannels = Arc<Mutex<Vec<DspBuffer>>>;
#[allow(dead_code)]
pub enum SigChannel {
    Channel0,
    Channel1,
    Channel2,
    Channel3
}

pub trait BufferTrait {
    fn get_my_vec_ref(&self) -> DspChannels;
    fn get_cond_var_ptr(&self) -> ReturnPair;
    fn wait_for_start(&mut self) -> ();
    fn end_of_processing(&mut self) -> ();
    fn get_fill_length(&self) -> usize;
}

// basic struct that will house common components
#[allow(dead_code)]
pub struct BasicBufferProcessor {

    // lock and set these 'I'm done' flags during construction
    my_cond_ptr: Arc<Mutex<bool>>,
    my_cond_var: Arc<Condvar>,
    my_buffer: DspChannels,

    // wait on (and check) these to even start during run()
    prev_cond_ptr: Arc<Mutex<bool>>,
    prev_cond_var: Arc<Condvar>,
    prev_buffer: DspChannels,

    fill_length: usize,
    name: String
}

impl BasicBufferProcessor {
    pub fn new(fill_size: usize, pab: Arc<Mutex<bool>>, pcv: Arc<Condvar>, pb: DspChannels, name: String) -> BasicBufferProcessor {
        BasicBufferProcessor {
            my_cond_ptr: Arc::new(Mutex::new(false)),
            my_cond_var: Arc::new(Condvar::new()),
            my_buffer: Arc::new(Mutex::new(Vec::new())),

            prev_cond_ptr: pab,
            prev_cond_var: pcv,
            prev_buffer: pb,

            fill_length: fill_size,
            name: name
        }
    }
}


impl BufferTrait for BasicBufferProcessor {
    fn get_my_vec_ref(&self) -> DspChannels {
        Arc::clone(&self.my_buffer)
    }

    fn get_cond_var_ptr(&self) -> ReturnPair {
        (Arc::clone(&self.my_cond_ptr), Arc::clone(&self.my_cond_var))
    }

    fn wait_for_start(&mut self){
        // not done, make sure we mark ourselves as 'incomplete'
        let mut finished = self.my_cond_ptr.lock().unwrap();
        *finished = false;

        // now wait for prev component to be done
        println!("{} waiting for start signal...", self.name);
        let mut prev_finished = self.prev_cond_ptr.lock().unwrap();
        while !*prev_finished {
            prev_finished = self.prev_cond_var.wait(prev_finished).unwrap();
        }
        println!("{} got start signal!!", self.name);
    }

    fn end_of_processing(&mut self) {
        // mark ourselves as done and notify next listener
        println!("{} marking itself as 'done'...", self.name);
        let mut finished = self.my_cond_ptr.lock().unwrap();
        *finished = true;
        self.my_cond_var.notify_one();
    }

    fn get_fill_length(&self) -> usize {
        self.fill_length
    }
}
