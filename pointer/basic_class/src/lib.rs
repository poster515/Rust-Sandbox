
#[derive(Debug)]
pub struct Class{
	pub name: String,
	pub value: i32,
    pub max_value: i32
}
    
impl Class {
    pub fn increment(&mut self) -> Result<i32, &'static str> {
        println!("Current value: {}, max_value: {}", self.value, self.max_value);

        let temp: i32 = self.value + 1;
        if temp >= self.max_value {
            return Err("Would increment over max value")
        }
        self.value = temp;
        Ok(temp)
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.value)
    }
}