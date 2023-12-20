


use num_traits::Num;
use num_traits::identities::zero;
use num_traits::identities::one;

pub trait Incrementable {
    fn increment(&mut self) -> ();
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GenericClass <T:>{
	name: String,
	value: T,
    max_value: T
}

impl<T: Num + std::cmp::PartialOrd + Copy> GenericClass <T>{
    pub fn new(iname: String, mv: T) -> GenericClass<T> {
        GenericClass {
            name: iname,
	        value: zero(),
            max_value: mv
        }
    }
}

impl <T: Num + std::fmt::Display> std::fmt::Display for GenericClass <T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.value, self.max_value)
    }
}

impl<T: Num + std::cmp::PartialOrd + Copy + std::fmt::Display> Incrementable for GenericClass <T>{
    fn increment(&mut self) -> () {

        println!("Current value: {}, max_value: {}", self.value, self.max_value);

        let temp: T = self.value + one();
        if temp >= self.max_value {
            return
        }
        self.value = temp;
    }
}

