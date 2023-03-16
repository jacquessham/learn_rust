fn main(){
	// Example 1
	let mut num = 10;
	if num >= 5 {
		println!("The number is greater than 5");
	} else if num > 3 {
		println!("The number is either 3 or 4");
	} else {
		println!("The number is less than 3");
	}

	// Example 2
	let gate = false;
	if gate {
		println!("The variable value is {gate}");
	} else {
		println!("Gate should not be {gate}");
	}

	// Example 3
	num -= 6;
	let num2 = if num >= 5 {"Greater than 5"} else if num > 3 {"Either 3 or 4"} else {"Less than 3"};
	println!("The number is {num2}");

	// Match Statement
	match num{
		1|3|5|7|9 => println!("{num} is an odd number"),
		2 => println!("This number is {num}"),
		_=> println!("{num} is not an odd number nor 2"),
	};
}