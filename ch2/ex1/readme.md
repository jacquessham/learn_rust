# Exercise 1: Guess game
Cargo Project for this lesson: <i>guess_game</i>
<br><br>
This is a classic beginner programming problem with the instruction coming from <a href="https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html">The Book chapter 2</a>. The goal is to generate a random integer between 1 and 100 and have player to guess the randomly generated number. 

## Syntax Explanations
### User's input
Utilizes <i>std::io</i> library, using the function <b><i>io.stdin().read_line(&mut guess).expect("Failed to read line")</i></b> to read user's keyboard input and assign to a variable.
<br><br>
<i>.read_line(&mut guess)</i> is the readline function along <i>guess</i> is the variable assigned to. Notice the <b><i>&</i></b> sign is a reference, which allows you access one piece of data wihtout needing to copy that data into memory multiple times. Without using a reference here, the code will fell. 
<br><br>
Also note that the <i>guess</i> should be redeclared in each loop.

### Data Convertion
<i>let guess: i8 = guess.trim().parse().expect("Please type a number!")</i> utilize three function to convert a string to an integer:
<ul>
	<li><i>trim()</i>: Trim the escape characters</li>
	<li><i>parse()</i>: Do the actual convertion</li>
	<li><i>expect()</i>: For message display if convertion error occurs</li>
</ul> 

### Comparsion
Utilizes <i>std::cmp::Ordering</i> to make comparison rather than if-statement with operators. Readability is one of the biggest reasons: 
<ul>
	<li>You cannot accidentally forget a case, it's guaranteed to be exhaustive.</li>
	<li>The condition is only written once, no need to "reverse" it or anything</li>
	<li>A match is just more maintainable than an if/else chain.</li>	
</ul>
The reason are referenced from <a href="https://stackoverflow.com/questions/48094170/when-to-use-stdcmpordering-instead-of-an-if-statement-in-rust">this Stack Overflow post</a>. The reasons are great for C++ users but those reasons do not make a huge advantage for users with Python background.