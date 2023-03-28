// Define a struct
struct Student{
	grad: bool,
	first_name: String,
	last_name: String,
	class: i32,
	gpa: f32
}

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

// A struct may defined without instance
struct Teacher;

// Function to pass referenced constuct variable, without using helper func
fn years_grad(curr_year:i32, student: &Student) -> i32 {
	curr_year - student.class
}

// Main Function
fn main(){
	let mut stu1 = Student{
		grad: false,
		first_name: String::from("Jacques"),
		last_name: String::from("Sham"),
		class: 2009,
		gpa: 4.0
	};

	stu1.grad = true;

	let stu1_yrs_grad = years_grad(2023, &stu1);

	println!("stu1's full name is {}", stu1.fullname());
	println!("stu1 has graduated for {stu1_yrs_grad} years", );
	println!("My comment to stu1 is {}",stu1.gpa_feedback());

	let _teach1 = Teacher;
}