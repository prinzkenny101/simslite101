//This is a SIMS application we'll be building this week 
//The project name is SIMS lite

/*
DAY 4
Task: Save user input to a file
To a text file
The program should be able to retreive data from the text file
*/

use std::io;

//Function for user input
fn input() -> String {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1:String = input1.trim().to_string();
    return input1; //Returning the value of input1
}

//Function for reading number 
fn read_number() -> f32 {
input().parse().expect("Invalid number")
}

//Function to detrmine grade
fn determine_grade(total: f32) -> char {
if total >= 70.0 {
    'A'
}else if total >= 60.0 {
    'B'
}else if total >= 50.0 {
    'C'
}else if total >= 45.0 {
    'D'
} else {
    'F'
}
}


//Main function
// *This block of code below is for the sign up module
//I'll create the log-in module soon
fn main() {
    println!("");
    println!("SIMS lite is now online 😁");
    println!("");
    println!("Enter your name");
    let user_name:String = input().to_lowercase();

    println!("");
    println!("Enter your matriculation number");
    let matric_num:String = input().to_uppercase();
    std::fs::File::create(&matric_num).expect("File creation failed");
    println!("Your own personal Data file has been created.");
    println!("Your file name is {}",matric_num);

    println!("");
    println!("Enter your faculty");
    println!("
    Choose your faculty from the options below
    - SMSS (School of management and social sciencesb)
    - SMC (School of Media and Communication)
    - SST (School of science and technology)
    ");
    let faculty:String = input().to_lowercase();

    println!("");
    println!("Enter your department");
    let department:String = input().to_lowercase();

    println!("");
    println!("Enter your age");
    let user_age:u64 = input().trim().parse().expect("Not a valid number");

    println!("");
    println!("Enter your admission year");
    let ad_year:u64 = input().trim().parse().expect("Not a valid number");

    println!("");
    println!("Enter your gender");
    println!("Note that gender can only be male or female");
    let user_gen:String = input().to_lowercase();
    
    let course_num = 0;
    let mut courses:Vec<String> = Vec::new();
    loop {
        courses.clear();

        println!("");
        println!("Enter the number of courses you are taking");
        println!("Note that the number of courses cannot be zero");
        let course_num:u64 = input().trim().parse().expect("Not a valid number");
        println!("");
        println!("Input the course codes of the courses you are taking!");
        for count in 0..course_num {
            println!("Enter course {}",count+1);
            let new_course = input().to_uppercase();
            courses.push(new_course);
            println!("");
        }
        println!("Your course are: \n");
        let mut count = 1;
        for i in &courses {
            println!("{} {}", count, i);
            count += 1;
        }
        println!("");
        println!("Would you like to change/edit the  courses you're taking ");
        println!("Input Y to change your courses or N to move on !");
        let option = input().to_lowercase();
        println!("");
        if option == "n" {
            println!("You have prompted to move on, moving on...");
            break;
        }else {
            println!("Starting over...");
        }
    }

    println!("");
    println!("Enter your student status");
    println!("Your options are either true or false");
    let user_status:bool = input().trim().to_lowercase().parse().expect("Invalid input");

    println!("");
    println!("Input your year of study");
    println!("e.g. if you're in year 1 input 1 and so on");
    let study_year:u64 = input().trim().parse().expect("Not a valid number");
    
    println!("You re successfully onboarded, welcome to the SIMS portal");
    println!("");
    println!("DASHBOARD");
    println!("");
    println!("Your information");
    println!("
              \nName:{}
              \nMatriculation number: {}
              \nFaculty:{}
              \nDepartment:{}
              \nAge:{}
              \nAdmissionnn year:{}
              \nGender:{}
              \nCourses offered:{}
              \nStudent status:{}
              \nYear of study:Year {}
    ",user_name,matric_num,faculty,department,user_age,ad_year,user_gen,course_num,user_status,study_year);

    let mut grades:Vec<String> = Vec::new();
    let mut total_scores:Vec<f32> = Vec::new();

    loop {
        grades.clear();
        total_scores.clear();
        
        println!("");
        println!("Start inputing the scores for your various courses");
        let a = courses.len();
        for i in 0..a {
            let course:&String = &courses[i];
            println!("{}",course);

            println!("Enter first test score out of 15\n");
            let test1 = read_number();

            println!("Enter second test score out of 15\n");
            let test2 =read_number();

            println!("Enter prticipation score out of 5\n");
            let participation =read_number();

            println!("Enter your exam score out of 65\n");

            let exam = read_number();
            let total = test1 + test2 + participation + exam;
            let final_grade = determine_grade(total).to_string();

            //The to_string() converts the char return type of the determine_grade function to a string 
            println!("Total score: {}\n", total);
            println!("Grade: {}\n", final_grade);
            println!("");

            grades.push(final_grade);
            total_scores.push(total);
            //The .push() method pushes what the user inputs into their respective vectors for storage
            }

            let b = courses.len();
            // Courses length is the length of the courses vector which will be th esme for the other vectors
            println!("The total summary of your scores are as follows");
            for i in 0..b {
                println!("");
                println!("Course:{} , Total score: {} , Final grade: {}",courses[i],total_scores[i],grades[i]);
            }
        println!("Is this summary correct or would you like to edit your scores ?");
        println!("Input Y to change your scores or N to move on !");
        let option2 = input().to_lowercase();
        println!("");
        if option2 == "n" {
            println!("You have prompted to move on, moving on...");
            break;
        }else {
            println!("Starting over...");
        }
      }
    
    }
    
    /*
    The basic logic for the application is as follows, user inputs his courses. After that he selects the course he wants to input the score for, then he selects the type of assessment he want to input. Then he inputs what the assessment eas graded over. Then the application converts whatever score was provided.
    I'll find a way to save each new users scores to a file and also retreuve the information to carry out other things like calculating CGPA and other things like that
    */



