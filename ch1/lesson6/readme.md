# Lesson 6: Loops
File for this lesson: <i>lesson6.rs</i>. 
<br><br>
There are 3 different loops available in Rust:
<ul>
	<li><i>loop</i></li>
	<li><i>while</i> loop</li>
	<li><i>for</i> loop</li>
</ul>

## Examples
### loop
<i>loop</i> is a loop that will enter the loop and break when a certain condition is met. An example is:

```
loop{
	println!("I am looping in a loop!");
	i += 1;
	if i >5 {break};
}
```

### while loop
<i>while</i> is a loop that will only enter the loop when a certain condition is met. An example is:

```
while j < 5{
	println!("I am looping in a while loop!");
	j += 1;
}
```

### for loop
<i>for</i> loop in Rust allows you to for loop in both a range and an array. The examples are:

```
// Example of a for loop using range
for _num in 0..4{
	println!("I am looping in a for loop!");
}
println!("Another example of using a for loop:");
for prime in arr {
	println!("{prime} is a prime number");
}
```

<br>
Note: Range in Rust lower bound inclusive and upper bound exclusive like Python, but it has a Ruby-like syntax like <b>0..4</b>