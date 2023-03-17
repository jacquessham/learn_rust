fn main(){
	print_elem(8, "Eight");

	let num1 = 8;
	let num2 = 9;
	let num = calculate_nums(num1, num2);
	println!("{num1} add {num2} yields to {num}");

}

fn print_elem(elem1: u8, elem2: &str){
	println!("The elements are {elem1} and {elem2}");
}

fn calculate_nums(num1: u8, num2: u8) -> u8 {
	num1 + num2
}