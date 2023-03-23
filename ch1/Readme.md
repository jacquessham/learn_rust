## Chapter 1: Basic Syntax
In this chapter, we will go over <a href="https://doc.rust-lang.org/book/title-page.html">The Book (The Rust Programming Language)</a> to learn the basic syntax in Rust.

## Lesson 1: Hello World
File for this lesson:  <i>main.rs</i><br>
It can be found in the <a href="https://github.com/jacquessham/learn_rust/tree/main/ch1/lesson1">Lesson 1 Folder</a>
<br><br>
In Rust, <b>println!()</b> is the print line statement.
<br><br>
Before execute a Rust script, you need to compile the script and run the compiled file afterward. Here is the example to run <i>main.rs</i>:

```
rustc main.rs
./main
```

## Lesson 2: Cargo
File for this lesson:  <i>main.rs</i><br>
It can be found in the <a href="https://github.com/jacquessham/learn_rust/tree/main/ch1/lesson2">Lesson 2 Folder</a>
<br><br>
In this lesson, we will go over how to utilize <i>Cargo</i> for build system and package management.

## Lesson 3: Variables, Data Types, and Comments
File for this lesson:  <i>lesson3.rs</i><br>
It can be found in the <a href="https://github.com/jacquessham/learn_rust/tree/main/ch1/lesson3">Lesson 3 Folder</a>
<br><br>
By default variables are <b>immutable</b>, and you have to use the keyword <b>let</b> to declare a variable. If you want to declare an immutable or mutable varibale, here is the syntax:

```
let x = 5;
let mut y = 6;
let y:i8 = 7;
let word = "Hello World!"; // This is &str data type
let word_str: String = "Hello World!".to_string(); // This is string
```

<br><br>
You may find the data type details of <i>integers</i>, <i>float</i>, <i>boolean</i>, <i>character</i>, <i>string</i>, <i>tuple</i>, <i>array</i>. In Rust:
<ul>
	<li><b>integers may be signed or unsigned: Unsigned integer means only positive number are accepted</b></li>
	<li><b>float can only be 32 or 64 bits</b></li>
	<li><b>tuples in Rust have a fixed length and fixed element data type</b></li>
</ul>

<br>
Note that when you declare a variable with double quotation, it is a <i>&str</i> data type, you should use <i>to_string()</i> function to convert to string.

## Lesson 4: If-statement
File for this lesson:  <i>lesson4.rs</i><br>
It can be found in the <a href="https://github.com/jacquessham/learn_rust/tree/main/ch1/lesson4">Lesson 4 Folder</a>
<br><br>
The if-statement syntax are <i>if{}</i>, <i>else if{}</i>, <i>else{}</i>. You may use a boolean variable for evalution. You may also use if-statement to assign a new variable.

## Lesson 5: Function
File for this lesson:  <i>lesson5.rs</i><br>
It can be found in the <a href="https://github.com/jacquessham/learn_rust/tree/main/ch1/lesson4">Lesson 5 Folder</a>
<br><br>
In Rust, the function sytnax is <b>fn()</b> followed by a function name and parameters. Like Java, the parameters are required to stated the data type.
<br>
You must declare <b>-></b> for functions which have return values and stated the returned data type, the return value is always the final block of a function.

## Lesson 6: Loops
File for this lesson:  <i>lesson6.rs</i><br>
It can be found in the <a href="https://github.com/jacquessham/learn_rust/tree/main/ch1/lesson4">Lesson 6 Folder</a>
<br><br>
There are 3 different loops available in Rust:
<ul>
	<li><i>loop</i></li>
	<li><i>while</i> loop</li>
	<li><i>for</i> loop</li>
</ul>
<br>
<i>for</i> loop in Rust allows you to for loop in both a range and an array. The syntax of a while loop is <b>while condition {}</b> and the syntax of a for loop is <b>for prime in arr {}</b> or <b>for i in 0..4</b>. Range in Rust lower bound inclusive and upper bound exclusive.

## Lesson 7: Structure (Class)
File for this lesson:  <i>lesson7.rs</i><br>
It can be found in the <a href="https://github.com/jacquessham/learn_rust/tree/main/ch1/lesson7">Lesson 6 Folder</a>
<br><br>
Coming soon...