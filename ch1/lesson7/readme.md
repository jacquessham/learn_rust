# Lesson 7: Struct (Class)
Struct is a class in Rust and allow you to create your data type for creating new objects.

## Syntax
Here is the syntax to declare a struct.

```
// Define a struct
struct Student{
	grad: bool,
	first_name: String,
	last_name: String,
	class: i32,
	gpa: f32
}

// A structure may have no instance
struct Teacher;

// Declare a variable
let mut stu1 = Student{
		grad: false,
		first_name: String::from("Jacques"),
		last_name: String::from("Sham"),
		class: 2009,
		gpa: 4.0
	};
```
<br><br>

You may use <b>impl</b> to implement helper function to a struct, use <b>&self</b> to reference the struct object:

```
// Multiple Helper functions in one bracket
impl Student{
	fn gpa_feedback(&self) -> &str{
		if self.gpa >= 3.5{
			"Great Student!"
		}
		else if self.gpa >= 3.0{
			"Good Student!"
		}
		else if self.gpa >= 2.0{
			"Not too bad"
		}
		else{
			"No comment"
		}
	}

	fn fullname(&self) -> String{
		format!("{} {}", self.first_name, self.last_name)
	}
}

// You can have one bracket to have only 1 helper function
impl Student{
	fn isGrad_str(&self) -> &str{
		match self.grad{
			true => "Graduated",
			false => "Not Graduated"
		}
	}
}
```

<br><br>
When writing a function reference a struct type, use <b>&(struct_name)</b> as data type in the parameter:

```
// Function to pass referenced constuct variable, without using helper func
fn years_grad(curr_year:i32, student: &Student) -> i32 {
	curr_year - student.class
}
```

## Examples
File to be used:  <i>lesson7.rs</i><br>
<br><br>
Running this file will define two new data type - <i>Student</i> and <i>Teacher</i>. Then, it will declare a Student object called <i>stu1</i> and call all the helper functions, ie, print full name, years after graduation, and comment based on his GPA. 