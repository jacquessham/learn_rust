# Lesson 4: If-statement
File for this lesson: <i>lesson4.rs</i>. 

## If-Statement
The if-statement syntax are <i>if</i>, <i>else if</i>, <i>else</i>. You may use a boolean variable for evalution. The examples are:

```
# Example 1
let num = 10;

if num >= 5 {
	println!("The number is greater than 5");
} else if num > 3 {
	println!("The number is either 3 or 4");
} else {
	println!("The number is less than 3");
}

# Example 2
let gate = false;

if gate {
	println!("The variable value is {gate}");
} else {
	println!("Gate should not be {gate}");
}
```

<br><br>
You may use if-statement to assign a new variable, like below:

```
let num2 = if num1 >= 5 {"Greater than 5"} else if num > 3 {"Either 3 or 4"} else {"Less than 3"};
```

<br>
But note that, the assigned value within the if-statement must be the same data type.

## Match Statement (Switch)
<i>match</i> is the syntax for Switch statement. You may use <i>=></i> for the logic, <i>\|</i> for separation for several condition and <i>\_</i> for else statement An example is:

```
match num{
	1|3|5|7|9 => println!("{num} is an odd number"),
	2 => println!("This number is {num}"),
	_=> println!("{num} is not an odd number nor 2"),
};
```
