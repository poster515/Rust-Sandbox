

use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::AtomicBool;

use num::complex::Complex;

type return_pair Arc::pair<AtomicBool, Condvar>;

pub trait dsp_path_member {
    fn get_my_vec_ref(&self) -> &Arc<Mutex<Vec<Vec<Complex>>>>;
    fn get_cond_var_ptr(&self) -> return_pair;
    fn wait_for_start() -> ();
    fn end_of_processing() -> ();
}

// basic struct that will house common components
pub struct BasicBufferProcessor {

    // lock and set these 'I'm done' flags during construction
    my_cond_ptr: AtomicBool;
    my_cond_var: Condvar;
    my_buffer: Arc<Mutex<Vec<Vec<Complex>>>>;

    // wait on (and check) these to even start during run()
    prev_cond_ptr: &AtomicBool;
    prev_cond_var: &Condvar;
    prev_buffer: Arc<Mutex<Vec<Vec<Complex>>>>;
}

impl BasicBufferProcessor {
    pub fn new(pab: &AtomicBool, pcv: &Condvar) -> BasicBufferProcessor {
        BasicBufferProcessor {
            my_cond_ptr: AtomicBool::new(false),
            my_cond_var: Condvar::new(),
            my_buffer: Arc::new(Mutex::new()),

            prev_cond_ptr: pab,
            prev_cond_var: pcv,
            prev_buffer: Arc<Mutex<Vec<Vec<Complex>>>>
        }
    }
}


impl dsp_path_member for BasicBufferProcessor {
    pub fn get_cond_var_ptr(&self) -> return_pair {
        Arc::clone(&self.cond_var)
    }

    pub fn wait_for_start(){
        // not done, make sure we mark ourselves as 'incomplete'
        my_cond_ptr.store(false, atomic::Ordering::Relaxed);

        // now wait for prev component to be done
        let mut prev_finished = prev_cond_ptr.load(Ordering::SeqCst);
        while !prev_finished {
            prev_finished = prev_cond_var.wait(prev_finished).unwrap();
        }
    }

    pub fn end_of_processing() {
        // mark ourselves as done and notify next listener
        my_cond_ptr.store(true, Ordering::SeqCst);
        my_cond_var.notify_one();
    }
}
