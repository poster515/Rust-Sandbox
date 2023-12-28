

use num_traits::Num;
use num::cast::AsPrimitive;
use num_traits::identities::one;

use std::f64::consts::PI;

use std::sync::{Arc, Mutex};

pub fn generate_square_wave_data<T: Num + std::ops::Neg<Output = T> + std::fmt::Display + Copy>(vec: &mut Vec<T>
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64){

	// get the current clock value
	let mut val: T;
	let value_guard = clock.lock().unwrap();
	let mut temp_clock_value: i8 = *value_guard;
	drop(value_guard);

	for i in 0..buffer_len {
		while temp_clock_value == 0 {
			temp_clock_value = *clock.lock().unwrap();
		}

		if (i as i64 % period) < (period/2) { 
			val = one::<T>();
		} else {
			val = -one::<T>();
		}
		(*vec).push(val);

		while temp_clock_value == 1 {
			temp_clock_value = *clock.lock().unwrap();
		}		
	}
}

pub fn generate_sine_wave_data(vec: &mut Vec<f64>
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64){

	// get the current clock value
	let mut val: f64;
	let value_guard = clock.lock().unwrap();
	let mut temp_clock_value: i8 = *value_guard;
	drop(value_guard);

	for i in 0..buffer_len {
		while temp_clock_value == 0 {
			temp_clock_value = *clock.lock().unwrap();
		}
		val = ((2.0 * PI * (i as f64 % period as f64)) / period as f64).sin();
		// val = ((2.0 * PI * (i as f64 % period as f64)) / period as f64).sin().as_();
		(*vec).push(val);

		while temp_clock_value == 1 {
			temp_clock_value = *clock.lock().unwrap();
		}		
	}
}

pub fn generate_triangle_wave_data<T: Num + std::ops::Neg<Output = T> + std::fmt::Display + Copy + 'static>(vec: &mut Vec<T>
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64)
		
		where i64: AsPrimitive<T> {

	// get the current clock value
	let mut val: T;
	let value_guard = clock.lock().unwrap();
	let mut temp_clock_value: i8 = *value_guard;
	drop(value_guard);

	for i in 0..buffer_len {
		while temp_clock_value == 0 {
			temp_clock_value = *clock.lock().unwrap();
		}
		if (i as i64 % period) < (period/2){
			val = ((4*(i as i64 % period))/period - 1).as_();
		} else {
			val = (3 - (4*(i as i64 % period))/period).as_();
		}
		(*vec).push(val);

		while temp_clock_value == 1 {
			temp_clock_value = *clock.lock().unwrap();
		}		
	}
}

pub fn generate_sawtooth_wave_data<T: Num + std::ops::Neg<Output = T> + std::fmt::Display + Copy + 'static>(vec: &mut Vec<T>
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64)
		
		where i64: AsPrimitive<T> {

	// get the current clock value
	let mut val: T;
	let value_guard = clock.lock().unwrap();
	let mut temp_clock_value: i8 = *value_guard;
	drop(value_guard);

	for i in 0..buffer_len {
		while temp_clock_value == 0 {
			temp_clock_value = *clock.lock().unwrap();
		}
		
		val = (((2 * (i as i64 % period)) / period) - 1).as_();
		(*vec).push(val);

		while temp_clock_value == 1 {
			temp_clock_value = *clock.lock().unwrap();
		}		
	}
}

