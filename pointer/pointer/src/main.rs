
use basic_class;
use incrementor;

fn main() -> () {
	let mut bc = basic_class::Class{ name: "NAME".to_string(), value: 1, max_value: 2 };

	let mut inc: incrementor::Incrementor = Default::default();

	inc.increment_class_value(&mut bc);

	println!("class: {}", bc);
}
