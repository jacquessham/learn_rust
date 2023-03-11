# Lesson 3: Variables, Data Types, and Comments
File for this lesson: <i>main.rs</i>.
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
