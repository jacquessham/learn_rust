# Lesson 3: Variables, Data Types, and Comments
File for this lesson: <i>main.rs</i>.
<br><br>
The materials of this lesson can be found in The Book Chapter <a href="https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html">3.1 Variables and Mutability</a>, <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html">3.2 Data Types</a>, <a href="https://doc.rust-lang.org/book/ch03-04-comments.html">3.4 Comments</a>.

## File
You may first compile <i>main.rs</i>. The executable will assign variables and print on the command line.

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
Coming soon...