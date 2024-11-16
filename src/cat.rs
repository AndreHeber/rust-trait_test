use std::fmt;
use std::ops::Add;

pub struct Cat {
	pub name: String,
	pub age: u8,
	pub hair_color: String,
	pub hair_length: u8,
	pub sound: String,
}

impl fmt::Display for Cat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "My cats's name is {}, the age is {} and the hair is {} with {}cm length and makes {}.", self.name, self.age, self.hair_color, self.hair_length, self.sound)
	}
}

impl Add<u8> for Cat {
    type Output = Cat;

    fn add(self, other: u8) -> Cat {
        Cat {
            name: self.name,
            age: self.age + other,
            hair_color: self.hair_color,
			hair_length: self.hair_length,
			sound: self.sound,
        }
    }
}

impl Add<&String> for Cat {
    type Output = Cat;

    fn add(self, other: &String) -> Cat {
        Cat {
            name: self.name,
            age: self.age,
            hair_color: self.hair_color,
			hair_length: self.hair_length,
			sound: self.sound + other,
        }
    }
}