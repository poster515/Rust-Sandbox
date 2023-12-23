

use num_traits::Num;
use num_traits::identities::one;

use std::sync::{Arc, Mutex};

pub fn generate_square_wave_data<T: Num + std::ops::Neg<Output = T> + std::fmt::Display>(vec: &mut Vec<T>
		, buffer_len: usize
		, clock: Arc<Mutex<i8>>
		, period: i64){
	let mut val: T;

	let value_guard = clock.lock().unwrap();
	let current_clock_value: i8 = *value_guard;
	let mut temp_clock_value: i8 = current_clock_value;
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

// template <typename T>
// pub fn generateSineWaveData(T ** buffer, const int channel){

// 	for (int i = 0; i < BUFFER_LEN; i++){
// 		bool updated = false;
// 		while(CLOCK == 0){}
// 		while(CLOCK == 1){
// 			if (!updated){
// 				updated = true;
// 				buffer[channel][i] = (T)sin((2 * PI * (i % N)) / N);
// 			}
// 		}
// 	}
// }

// template <typename T>
// pub fn generateTriangleWaveData(T ** buffer, const int channel){
// 	for (int i = 0; i < BUFFER_LEN; i++){
// 		bool updated = false;
// 		while(CLOCK == 0){}
// 		while(CLOCK == 1){
// 			if (!updated){
// 				updated = true;
// 				buffer[channel][i] = (i % N) < (N/2) ? (T)(4*(i % N))/N - 1 : 3 - (T)(4*(i % N))/N;
// 			}
// 		}
// 	}
// }

// template <typename T>
// pub fn generateSawtoothWaveData(T ** buffer, const int channel){
// 	for (int i = 0; i < BUFFER_LEN; i++){
// 		bool updated = false;
// 		while(CLOCK == 0){}
// 		while(CLOCK == 1){
// 			if (!updated){
// 				updated = true;
// 				buffer[channel][i] = ((T)(2 * (i % N)) / N) - 1;
// 			}
// 		}
// 	}
// }

