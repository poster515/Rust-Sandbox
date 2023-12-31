

use std::sync::{Arc, Mutex, Condvar};

use num_complex::Complex;

pub type ReturnPair = (Arc<Mutex<bool>>, Arc<Condvar>);

#[allow(dead_code)]
pub enum SigChannel {
    Channel0,
    Channel1,
    Channel2,
    Channel3
}

pub trait BufferTrait {
    fn get_my_vec_ref(&self) -> Arc<Mutex<Vec<Vec<Complex<f64>>>>>;
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
    my_buffer: Arc<Mutex<Vec<Vec<Complex<f64>>>>>,

    // wait on (and check) these to even start during run()
    prev_cond_ptr: Arc<Mutex<bool>>,
    prev_cond_var: Arc<Condvar>,
    prev_buffer: Arc<Mutex<Vec<Vec<Complex<f64>>>>>,

    fill_length: usize
}

impl BasicBufferProcessor {
    pub fn new(fill_size: usize, pab: Arc<Mutex<bool>>, pcv: Arc<Condvar>) -> BasicBufferProcessor {
        BasicBufferProcessor {
            my_cond_ptr: Arc::new(Mutex::new(false)),
            my_cond_var: Arc::new(Condvar::new()),
            my_buffer: Arc::new(Mutex::new(Vec::new())),

            prev_cond_ptr: pab,
            prev_cond_var: pcv,
            prev_buffer: Arc::new(Mutex::new(Vec::new())),

            fill_length: fill_size
        }
    }
}


impl BufferTrait for BasicBufferProcessor {
    fn get_my_vec_ref(&self) -> Arc<Mutex<Vec<Vec<Complex<f64>>>>> {
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
        let mut prev_finished = self.prev_cond_ptr.lock().unwrap();
        while !*prev_finished {
            prev_finished = self.prev_cond_var.wait(prev_finished).unwrap();
        }
    }

    fn end_of_processing(&mut self) {
        // mark ourselves as done and notify next listener
        let mut finished = self.my_cond_ptr.lock().unwrap();
        *finished = true;
        self.my_cond_var.notify_one();
    }

    fn get_fill_length(&self) -> usize {
        self.fill_length
    }
}
