use std::io;  // using standard input/output 

//funtion for input
fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid input");
    let input: String = input.trim().to_string();  
    input    //return the function value
}

//SUMMING STUDENT'S TOTAL SCORES
fn sum(f_score: f32, s_score: f32, p_mark: f32, exam: f32) -> f32 {
    f_score + s_score + p_mark + exam
}

fn main() {

    let mut student_list: Vec<(u8,String,String,String,f32,f32,f32,f32,f32,&str)> = Vec::new();

    loop {

    println!("--- Welcome to student Information Management System ---");  //printing welcoming message
    println!("Enter your ID:");
    let id: u8 = input().parse().expect("Error, re-check it");

    println!("Enter your name:");  //name 
    let name:String = input(); 

    println!("Enter your Department:"); //department 
    let department : String = input().parse().expect("Error, re-check it");

    println!("Enter your Course Code:");  //course code
    let course_code:String = input();

    println!("Enter your First test score:");  //first test
    let f_score: f32 = input().parse().expect("Error, re-check it");

    println!("Enter your Second test score:");  //second test
    let s_score: f32 = input().parse().expect("Error, re-check it");

    println!("Enter your participation mark:");  //participation mark
    let p_mark: f32 = input().parse().expect("Error, re-check it");

    println!("Enter your exam score:");  //exam 
    let exam: f32 = input().parse().expect("Error, re-check it");

    let total = sum(f_score, s_score, p_mark, exam);

      // CONDITIONAL STATEMENT FOR GRADING
    let grade = if total >= 70.0 {
        "A"
    } else if total >= 60.0 {
        "B"
    } else if total >= 50.0 {
        "C"
    } else if total >= 45.0 {
        "D"
    } else {
        "F"
    };

    println!("Your information has been recorded");

    let student_info = (id, name.clone(), department.clone(), course_code.clone(), f_score, s_score, p_mark, exam, total, grade);
    student_list.push(student_info);

    //printing the details 
    println!("
        ID: {}\n 
        Name: {}\n
        Department: {}\n
        Course Code: {}\n
        First test score: {}\n
        Second test score: {}\n
        Participation mark: {}\n
        Exam score: {}\n
        Your total score is: {}\n
        Your final grade is: {}",
        id, name, department, course_code, f_score, 
        s_score, p_mark, exam, total, grade);

    println!("Do you want to add another student? (yes/no): ");
    let choice = input();

    if choice != "yes" {
        break;
      }

    }


}

/* So because of time, I couldn't think out a way to be able to loop the course for a specific
student before moving on to another student.
Also, there should be a condition for ensuring that the two tests are over 15 respectively, 
participation mark is over 5, exam is over 65 and the total is over 100 and declare error
if the user puts anything more than that. */