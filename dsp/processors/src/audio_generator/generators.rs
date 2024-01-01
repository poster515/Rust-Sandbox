
use num_complex::Complex;
use std::f64::consts::PI;

use std::sync::{Arc, Mutex};

use bb_processor::*;

pub fn generate_square_wave_data(vec: DataBuffer
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64){

	// get the current clock value
	let mut val: Complex<f64>;
	let value_guard = clock.lock().unwrap();
	let mut temp_clock_value: i8 = *value_guard;
	drop(value_guard);

	let v: &mut Vec<Complex<f64>> = &mut vec.lock().unwrap();
	for i in 0..buffer_len {
		while temp_clock_value == 0 {
			temp_clock_value = *clock.lock().unwrap();
		}

		if (i as i64 % period) < (period/2) { 
			val = Complex::new(1.0, 0.0);
		} else {
			val = Complex::new(-1.0, 0.0);
		}

		v[i] = val;

		while temp_clock_value == 1 {
			temp_clock_value = *clock.lock().unwrap();
		}
	}

	println!("generate_square_wave_data done!!");
}

pub fn generate_sine_wave_data(vec: DataBuffer
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64){

	// get the current clock value
	let mut val: Complex<f64>;
	let value_guard = clock.lock().unwrap();
	let mut temp_clock_value: i8 = *value_guard;
	drop(value_guard);

	let v: &mut Vec<Complex<f64>> = &mut vec.lock().unwrap();

	for i in 0..buffer_len {
		while temp_clock_value == 0 {
			temp_clock_value = *clock.lock().unwrap();
		}
		// val = ((2.0 * PI * (i as f64 % period as f64)) / period as f64).sin();
		val = Complex::new(((2.0 * PI * (i as f64 % period as f64) / period as f64)).sin(), 0.0);
		v[i] = val;

		while temp_clock_value == 1 {
			temp_clock_value = *clock.lock().unwrap();
		}		
	}
	println!("generate_sine_wave_data done!!");
}

pub fn generate_triangle_wave_data(vec: DataBuffer
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64){

	// get the current clock value
	let mut val: Complex<f64>;
	let value_guard = clock.lock().unwrap();
	let mut temp_clock_value: i8 = *value_guard;
	drop(value_guard);

	let v: &mut Vec<Complex<f64>> = &mut vec.lock().unwrap();

	for i in 0..buffer_len {
		while temp_clock_value == 0 {
			temp_clock_value = *clock.lock().unwrap();
		}
		if (i as i64 % period) < (period/2){
			val = Complex::new(((4*(i as i64 % period))/period - 1) as f64, 0.0);
		} else {
			val = Complex::new((3 - (4*(i as i64 % period))/period) as f64, 0.0);
		}
		v[i] = val;

		while temp_clock_value == 1 {
			temp_clock_value = *clock.lock().unwrap();
		}		
	}
	println!("generate_triangle_wave_data done!!");
}

pub fn generate_sawtooth_wave_data(vec: DataBuffer
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64){

	// get the current clock value
	let mut val: Complex<f64>;
	let value_guard = clock.lock().unwrap();
	let mut temp_clock_value: i8 = *value_guard;
	drop(value_guard);

	let v: &mut Vec<Complex<f64>> = &mut vec.lock().unwrap();

	for i in 0..buffer_len {
		while temp_clock_value == 0 {
			temp_clock_value = *clock.lock().unwrap();
		}
		
		val = Complex::new((((2 * (i as i64 % period)) / period) - 1) as f64, 0.0);
		v[i] = val;

		while temp_clock_value == 1 {
			temp_clock_value = *clock.lock().unwrap();
		}		
	}
	println!("generate_sawtooth_wave_data done!!");
}

