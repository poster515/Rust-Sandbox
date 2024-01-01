
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time};

use bb_processor::{HalfPipe, DataBuffer};
use processors::{AudioGenerator, FFT};
use bb_processor::DspPathMember;

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    let mut vector: Vec<HalfPipe> = Default::default();
    let buffer_len: usize = 8;
    let period: i64 = 4;

    // clock used to tick throughout the other generator threads
    let clock = Arc::new(Mutex::new(0 as i8));
    
    let buff_len_copy = buffer_len;
    let clk_ref = Arc::clone(&clock);

    let mut dsp_comps_spawned: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let dsp_wait = Arc::clone(&dsp_comps_spawned);

    let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();

    let clk_thread = thread::spawn(move || {
        // wait until all dsp comps marked as 'ready'
        println!("main() clock thread waiting for start signal...");
        while !dsp_wait.load(Ordering::SeqCst){
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
        }
        println!("main() clock thread got start signal!! running...");
        // then run the clock - two transitions for each element in buffer
        for _ in 0..(2 * buff_len_copy) {
            let ten_millis = time::Duration::from_millis(100);
            thread::sleep(ten_millis);
            
            let mut value_guard = clk_ref.lock().unwrap();
            (*value_guard) = 1 - (*value_guard);
		};
        println!("main() clock thread done!!");
    });
    // create dummy halfpipes
    for i in 0..4 {
        let dummy_data: DataBuffer = Arc::new(Mutex::new(Vec::new()));
        let i_bool = Arc::new(Mutex::new(false));
        let i_cv = Arc::new(Condvar::new());
        vector.push(HalfPipe::new(dummy_data, i_bool, i_cv))
    }
    let mut audio_generator: AudioGenerator = AudioGenerator::new(buffer_len, &vector);
    let mut freq_transformer: FFT = FFT::new(buffer_len, &audio_generator.get_output_halfpipes());

    let ag_clk = Arc::clone(&clock);
    thread_handles.push(thread::spawn(move || {
        audio_generator.run(ag_clk, period);
    })); 

    let fft_clk = Arc::clone(&clock);
    thread_handles.push(thread::spawn(move || {
        freq_transformer.run(fft_clk, period);
    }));
    
    // let AG know it can start waiting on clock
    for i in 0..4 {
        *vector[i].data_ready.lock().unwrap() = true;
        vector[i].cond_var.notify_all();
    }

    // now start the clock
    dsp_comps_spawned.store(true, Ordering::SeqCst);
    clk_thread.join().unwrap();

    while thread_handles.len() > 0 {
        let cur_thread = thread_handles.remove(0); 
        cur_thread.join().unwrap();
    }
}
