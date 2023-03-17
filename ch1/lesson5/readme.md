# Lesson 5: Function
File for this lesson: <i>lesson5.rs</i>. 

## Function
In Rust, the function sytnax is <b>fn()</b> followed by a function name and parameters. Like Java, the parameters are required to stated the data type.
<br>
You must declare <b>-></b> for functions which have return values and stated the returned data type, the return value is always the final block of a function.
<br><br>
Examples are:

```
fn print_elem(elem1: u8, elem2: &str){
	println!("The elements are {elem1} and {elem2}");
}

fn calculate_nums(num1: u8, num2: u8) -> u8 {
	num1 + num2
}
```