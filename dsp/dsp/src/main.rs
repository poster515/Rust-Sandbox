
use std::sync::{Arc, Mutex};
use std::{thread, time};

use generator::generate_square_wave_data;

fn main() {
    let mut vector: Vec<f64> = vec![];
    let buffer_len: usize = 16;
    let period: i64 = 4;

    // clock used to tick throughout the other generator threads
    let clock = Arc::new(Mutex::new(0 as i8));
    
    let buff_len_copy = buffer_len;
    let clk_ref = Arc::clone(&clock);

    let thread_handle = thread::spawn(move || {
        for _ in 0..(2 * buff_len_copy) {

            let ten_millis = time::Duration::from_millis(100);
            thread::sleep(ten_millis);

            let mut value_guard = clk_ref.lock().unwrap();
            (*value_guard) = 1 - (*value_guard);
		};
    });

    let clk_ref2 = Arc::clone(&clock);
    generate_square_wave_data(&mut vector, buffer_len, clk_ref2, period);
    match thread_handle.join() {
        Ok(_) => println!("Joined clock thread"),
        Err(_) => println!("Thread panicked!!")
    }

    println!("{:?}", vector);
}
