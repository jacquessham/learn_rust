fn main(){
	// Example of a loop
	println!("Example of using a loop:");
	let mut i = 0;
	loop{
		println!("I am looping in a loop!");
		i += 1;
		if i >5 {break};
	}

	// Example of a while loop
	println!("Example of using a while loop:");
	let mut j = 0;

	while j < 5{
		println!("I am looping in a while loop!");
		j += 1;
	}

	// Example of a for loop using range
	println!("Example of using a for loop:");
	for _num in 0..4{
		println!("I am looping in a for loop!");
	}
	println!("Another example of using a for loop:");
	// Example of a for loop in an array
	let arr = [2,3,5,7,11];
	for prime in arr {
		println!("{prime} is a prime number");
	}
}