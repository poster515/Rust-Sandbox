
use std::sync::{Arc, Mutex, Condvar};
use std::{thread, time};

use num_complex::Complex;

use processors::{AudioGenerator, DspPathMember};

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    let mut vector: Vec<Complex<f64>> = vec![];
    let buffer_len: usize = 16;
    let period: i64 = 4;

    // clock used to tick throughout the other generator threads
    let clock = Arc::new(Mutex::new(0 as i8));
    
    let buff_len_copy = buffer_len;
    let clk_ref = Arc::clone(&clock);

    let buffer_len: usize = 128;
    let main_start_flag: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let main_cond_var: Arc<Condvar> = Arc::new(Condvar::new());

    let mut audio_generator: AudioGenerator = AudioGenerator::new(buffer_len, main_start_flag, main_cond_var);
    audio_generator.run(clk_ref.clone(), period); 
    
    let thread_handle = thread::spawn(move || {
        for _ in 0..(2 * buff_len_copy) {

            let ten_millis = time::Duration::from_millis(100);
            thread::sleep(ten_millis);

            let mut value_guard = clk_ref.lock().unwrap();
            (*value_guard) = 1 - (*value_guard);
		};
    });

    thread_handle.join().unwrap();
}
