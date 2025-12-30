use std::io;  // using standard input/output 

//function for calculating total score
fn sum(f_score: f32, s_score: f32, t_score: f32) -> f32 {
    f_score + s_score + t_score
}

fn average(f_score: f32, s_score: f32, t_score: f32) -> f32 {
    sum(f_score, s_score, t_score) / 3.0
}

//function for input
fn input() -> String {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let input1: String = input1.trim().to_string();  
    input1    //retun the function value
}

fn main() {
    println!("--- Welcome to student Information Management System ---");  //printing welcoming message
    println!("Enter your ID:");
    let id: u8 = input().parse().expect("Error, re-check it");

    println!("Enter your name:");  //name 
    let name:String = input(); 

    println!("Enter your Department:"); //department 
    let department : String = input();

    println!("Enter your First Course score:");  //first course
    let f_score: f32 = input().parse().expect("Error, re-check it");

    println!("Enter your Second Course score:");  //second course
    let s_score: f32 = input().parse().expect("Error, re-check it");

    println!("Enter your Third Course score:");  //third course 
    let t_score: f32 = input().parse().expect("Error, re-check it");

    //total score
    let total = sum(f_score, s_score, t_score);

   //average score
    let average = average(f_score, s_score, t_score);
    
    //printing the details 
    println!("ID: {}\n   
        Name: {}\n
        Department: {}\n
        First course score: {}\n
        Second course score: {}\n
        Third course score: {}\n,
        Total score: {}\n,
        Average score: {:.2}",
        id, name, department, f_score, s_score, t_score, total, average);
}

/* My bad today, I didn't consider some things before I started coding, hence why this code 
doesn't have grading and all, I'll do better tomorrow */