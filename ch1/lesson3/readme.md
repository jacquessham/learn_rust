# Lesson 3: Variables, Data Types, and Comments
File for this lesson: <i>lesson3.rs</i>.
<br><br>
The materials of this lesson can be found in The Book Chapter <a href="https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html">3.1 Variables and Mutability</a>, <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html">3.2 Data Types</a>, <a href="https://doc.rust-lang.org/book/ch03-04-comments.html">3.4 Comments</a>.

## File
You may first compile <i>lesson3.rs</i>. The executable will assign variables with the basic way and shadowing, a constant, and print on the command line.

## Variables
### The Basics
Variables in Rust are setup differently with Python. By default variables are <b>immutable</b>, and you have to use the keyword <b>let</b> to declare a variable. A simple example looks like:

```
let x = 5;
```
<br>
Note that the keyword <b>let</b> is required to declare a variable. Rust deliberately separates between declaration and assignment. Let's say if you want to reassign an existing variable, you have to declare <b>mut</b> to allow the variable be mutable.

```
let mut y = 6;
println!("The value of y is {y}");
y = 7;
println!("Now y is {y}");
```
<br>
If you did not declare <b>mut</b> a variable and reassign an existing variable, the program could not compile and return an error. 
<br><br>
Like Java, single quotations are for characters and double quotations are for string. You may put <b>{}</b> to pass a variable. 

### Constants
Although both constants and immutable variables do not allow to alter the assignment, there are some difference between both in the backend: 
<ul>
	<li>Constant is computed at compilation time (During when the script is complied), therefore the run time is faster and it is not required to compute again in the executables</li>
	<li>Immutable variables are always computed during run time in the executables</li>
</ul>
<br>
A good example is to declare a constant to contain how many seconds in an hour:

```
const SEC_IN_HOUR: u32 = 60 * 60;
```
<br>
Rust's constant naming convention is to use all uppercase with underscores between words.

### Shadowing
Shadowing is beyond incremental or decremental assignment. It is defined as the later variable is built on top of the earlier variable to take extra action. It is similar to the concept of the relationship between a super class and a subclass, but on a variable level. In Rust, you may use shadowing in an immutable variable. For example:

```
let x = 5;
let x = x + 1;
```
<br>
Although <i>x</i> is immutable, Rust allows you to incremental add 1 to the existing immutable <i>x</i>. When you apply shadowing, you need to place the keyword <b>let</b> before the new assignment.
<br><br>
The reason shadowing is similar to the concept of the relationship between a super class and a subclass because if a shadowed variable has a reassignment in a bracket, the change only taken place locally in the bracket. Once the program exit the bracket, the change is not been found in the variable outside of the bracket. For example:

```
let x = 5;
let x = x + 1;
println!("Now, the value of x is: {x}");
{
	let x = x * 2;
	println!("Now, the value of x is: {x}");
}
println!("Once I have exited the bracket, the value of x is: {x}");
```

<br>
After <i>x</i> is incrementally added 1 to 5, the first print statment will print <i>x</i> as 6. Once the program entered the bracket, a new <i>x</i> would overshadows the older <i>x</i> and incrementally mutiple by 2 locally. So the second print statement will print <i>x</i> as 12. Finally, when the program exit the bracket, the overshadowed <i>x</i> in the bracket will be deleted, so the "older" <i>x</i> is not been overshadowed. The third print statement would print <i>x</i> as 6 as before.
<br><br>
Shadowing also allow you change the data type when the variable is immutable. However, it will give you an error when the variable is mutable. For example:

```
let word = "hello";
let word = word.len();
```

## Data Type
The basic data type in Rust are:
<ul>
	<li>Integer (Signed/Unsigned)</li>
	<li>Float</li>
	<li>Boolean</li>
	<li>Character</li>
	<li>Tuple</li>
	<li>Array</li>
</ul>

<br>
You may assign a data type with a colon after the variable name, like below:

```
let num1: u8 = 12;
```
<br>
Rust is a statiically typed language, which means the data type must be defined at compile time. The compiler usually infer what data type we want to use. In many cases, you are able to convert data type, such as from string to integer. If so, use <i>.parse().expect("Not a number!")</i> like below:

```
let num1: u8 = "12".parse().expect("Not a number!")
```

### Integer
In Rust, there are signed and unsigned integer. The difference between those are whether the variable is possible for the number be negative. <b>Unsigned means integer is possitive only</b>. <b>i8</b> means signed 8-bit while <b>u8</b> means unsigned 8-bit. Example to assign both assigned and unassigned integers are:

```
let num0: i8 = -8;
let num1: u8 = 12;
```

You may use different length: 8, 16, 32, 64, 128. According to some discussion, there is no performance difference in terms of arithmetic computation between signed and unsigned integer in Rust (It means a difference in C/C++).

### Float
You may use <b>f32</b> to represent 32 bit float number, <b>float is only offering 32 or 64 bits</b>. An example is

```
let num2: f32 = 10.8;
```

<br>
The numeric operations includes addition, subtraction, multiplication, division, and remainder:

```
let num3 = 1 + 1; // Addition
let num4 = 4 - 3; // Subtraction
let num5 = 5 * 5; // Multiplication
let num6 = 4.5/3.2 // Division to Float
let num7 = 7/3 // Division to Integer, num7 becomes 2
let num8 = 8/5 // Reminder, num8 becomes 3
```

### Boolean
Both <i>true</i> and <i>false</i> are belongs to Boolean in Rust, and both values are spelled in lowercases. The syntax of the Boolean type is <b>bool</b>.

```
let gate: bool = true;
```

### Character
Rust does differentiate between character and string. Single quotations are used for characters and double quotations are used for string. The syntax of the Character type is <b>char</b>.

```
let ch: char = 'A';
```

### String
Rust requires double quotations to declare string. There are 3 types:
<ul>
	<li><i>String</i>: A dynamic heap string type, like <i>Vec</i>. You may modify the string data because it is mutable</li>
	<li><i>str</i>: Immutable sequence of UTF-8. It can only handle it behind a pointer</li>
	<li><i>&str</i>: Normally called a "string slice". You only need it to view of a string as it is immutable</li>
</ul>

An example is:

```
let s = "Hello World!" // This is &str
let t = "Hello World!".to_string() // Convert &str to String
```
<br>
Note: If you declare a variable with double quotation, you need to convert it to string with <i>to_string()</i> function.


### Tuple
Rust also offers tuple as data type. However, tuples in Rust have a fixed length: Once declared, the tuple cannot grow or shrink in size. An example is:

```
let a: (u8, f32, String) = (12, 8.32, "Hello World!");
println!("The tuple is {:?}", a)
```

<br>
When printing whole tuple in <i>println!()</i>, place <i>{:?}</i> and tuple as a second input like the above example. And here is the example to assign tuple values into 3 variables:

```
let (x,y,z) = tup;
```

### Array
Unlike in Python, but like Java, every element of a Rust array is required to have the same data type and a fixed length. Here is the example:

```
let arr1 = [1,2,3,4,5];
let arr2: [u8; 6] = [1,2,3,4,5,6]; // [type, length]
println!("The whole array of arr1 is {:?}", arr1); // To print array
```

<br>
Note: If you come from a Python background, you are not able to append or remove element from an array. If you want to do so, you would want to use a <i>Vec</i>.

<br><br>
Rust array is 0-based. You may access an element like below:

```
let elem1 = arr2[0];
let elem2 = arr2[1];
```

## Comment
In Rust, comments are simply using two slashes like below:

```
// Assign num10 be 10 in unsigned integer 8
let num10: u8 = 10
```

