mod dog;
mod cat;
mod speek;
use speek::Speek;

fn main() {
	let mydog = dog::Dog {
		name: "Rusty",
		age: 8,
		breed: "Golden Retriever",
	};
	let mycat = cat::Cat {
		name: String::from("Misty"),
		age: 3,
		hair_color: String::from("Black"),
		hair_length: 2,
		sound: String::from("Moo!"),
	};

	println!("{}", mydog);
	print!("My cat's name is {} and she is a {} cat.", mycat.name, mycat.hair_color);
	println!("My cat's sound is {}", mycat.speek());
	
	let mut mydog2 = mydog;
	mydog2.name = "Rusty Jr.";
	println!("My dog's name is {} and he is a {}. My sound is {}", mydog2.name, mydog2.breed, mydog2.speek());
	println!("My dog's name is {} and he is a {}. My sound is {}", mydog.name, mydog.breed, mydog.speek());

	let mut my_older_cat = mycat + 2;
	my_older_cat = my_older_cat + &String::from(" Meow!");
	println!("{}", my_older_cat);
}
