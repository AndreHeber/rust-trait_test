use std::fmt;
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Dog {
	pub name: &'static str,
	pub age: u8,
	pub breed: &'static str,
}

impl fmt::Display for Dog {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "My dog's name is {}, the age is {} and he is a {}.", self.name, self.age, self.breed)
	}
}

impl Add for Dog {
	type Output = Dog;
	fn add(self, other: Dog) -> Dog {
		Dog {
			name: self.name,
			age: self.age + other.age,
			breed: self.breed,
		}
	}
}
