struct Student{
	grad: bool,
	first_name: String,
	last_name: String,
	class: i32,
	gpa: f32
}

struct Teacher;

fn years_grad(curr_year:i32, student: &Student) -> i32 {
	curr_year - student.class
}

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

	println!("stu1 has graduated for {stu1_yrs_grad} years", );



	let teach1 = Teacher;
}