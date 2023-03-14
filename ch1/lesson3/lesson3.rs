fn main(){
	// Assigning a immutable variable
	let x = 5;
	println!("The value of x is: {x}");
	println!("Now I am adding 1 to x");
	let x = x + 1;
	println!("Now, the value of x is: {x}");
	// Show shadown
	{
		println!("Now I am multiple x to 2 in a bracket");
		let x = x * 2;
		println!("Now, the value of x is: {x}");
	}
	println!("Once I have exited the bracket, the value of x is: {x}");

	let mut y = 6;
	println!("The value of y is {y}");
	y = 7;
	println!("Now y is {y}");

	// Assigning a constant
	const SEC_IN_HOUR: u32 = 60 * 60;
	println!("There are {SEC_IN_HOUR} seconds in an hour.");

	// Showing different data
	let word = "hello";
	println!("The word is {word}.");
	let word = word.len();
	println!("It has {word} characters.");

	let num0: i8 = -8;
	println!("num0 is {num0}");

	let num1: u8 = "12".parse().expect("Not a number!");
	println!("num1 is {num1}");

	let num2: f32 = 10.8;
	println!("num1 is {num2}");

	let ch: char = 'A';
	println!("I have a letter {ch}");

	let gate: bool = true;
	println!("Gate is {gate}");

	// Tuples
	let a: (u8, f32, String) = (12, 8.32, "Hello World!".to_string());
	println!("The tuple declared is {:?}", a);

	let (_x, y, _z) = a;
	println!("The second tuple value is {y}");

	let arr1 = [1,2,3,4,5];
	println!("The whole array of arr1 is {:?}", arr1);

	let arr2: [u8; 6] = [1,2,3,4,5,6];
	let elem1 = arr2[0];
	let elem2 = arr2[1];
	println!("The first two elements in arr2 are {elem1} and {elem2}");


}