use crate::dog;
use crate::cat;

pub trait Speek {
	fn speek(&self) -> String;
}

impl Speek for dog::Dog {
	fn speek(&self) -> String {
		"Woof!".to_string()
	}
}

impl Speek for cat::Cat {
	fn speek(&self) -> String {
		self.sound.clone()
	}
}