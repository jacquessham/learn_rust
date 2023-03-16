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
y = 7;
```

<br><br>
You may find the data type details of <i>integers</i>, <i>float</i>, <i>boolean</i>, <i>character</i>, <i>string</i>, <i>tuple</i>, <i>array</i>. In Rust:
<ul>
	<li><b>integers may be signed or unsigned: Unsigned integer means only positive number are accepted</b></li>
	<li><b>float can only be 32 or 64 bits</b></li>
</ul>

<br>
Examples to assign different data types are:

```
let num0: i8 = -8;
let num1: u8 = 12;
let num2: f8 = 10.8;
let gate: bool = true;
let ch: char = 'A';
let a: (u8, f32, String) = (12, 8.32, "Hello World!".to_string());
let arr2: [u8; 6] = [1,2,3,4,5,6];
```

<br>
Note that when you declare a variable with double quotation, it is a <i>&str</i> data type, you should use <i>to_string()</i> function to convert to string.

## Lesson 4: If-statement
Coming Soon...

## Lesson 5: Function
Coming Soon... 

## Lesson 6: Structure (Class)
Coming soon...