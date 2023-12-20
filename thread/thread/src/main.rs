
use std::thread;

use generic_class::Incrementable;


fn main() {
	let mut bc = generic_class::GenericClass::new("NAME".to_string(), 200);

	let thread_join_handle = thread::spawn(move || {
		for _ in 0..10 {
			bc.increment();
		}
		
		println!("class: {}", bc);
	});


    let result = thread_join_handle.join();
	match result {
		Ok(_) => {
            println!("Successfully incremented!")
        },
		Err(e) => {
            println!("Could not increment?")
        },
	}
	
}
