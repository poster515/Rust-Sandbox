
use basic_class;

#[derive(Default)]
pub struct Incrementor {
	num_increments: i32
}

impl Incrementor {
	pub fn increment_class_value(&mut self, bc: &mut basic_class::Class) -> () {
		let result = bc.increment();
		match result {
			Ok(v) => {
                println!("Successfully incremented. Value: {v:?}");
                self.num_increments += 1;
            },
			Err(e) => {
                println!("Could not increment: {e:?}")
            },
		}
		
		println!("Total increments: {:?}", self.num_increments)
	}

    pub fn get_increments(&self) -> i32 {
        self.num_increments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bc = basic_class::Class{ name: "NAME".to_string(), value: 1, max_value: 2 };
        let mut inc: Incrementor = Default::default();

        inc.increment_class_value(&mut bc);

        assert_eq!(inc.get_increments(), 0);
    }
}
