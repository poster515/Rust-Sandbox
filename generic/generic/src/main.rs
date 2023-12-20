
use generic_class::Incrementable;

fn main() -> () {
	let mut bc = generic_class::GenericClass::new("NAME".to_string(), 2);
	bc.increment();
	println!("class: {}", bc);

	let mut bc1 = generic_class::GenericClass::new("NAME".to_string(), 2.5);
	bc1.increment();
	println!("class: {}", bc1);
}
