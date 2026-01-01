use std::io;  // using standard input/output 

//funtion for input
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

//phy101
    println!(" --------PHY101 scores --------");
    println!("Enter your PHY101 1st CA score: ");
    let phy1ca1 : f32 =  input().parse().expect("Error, re-check it"); //phy first test 
    
    println!("Enter your PHY101 2nd CA score: ");
    let phy1ca2 : f32 =  input().parse().expect("Error, re-check it"); //phy 2nd test 

    println!("Enter your PHY101 Participation mark: ");
    let p_mark : f32 =  input().parse().expect("Error, re-check it"); //phy participation mark 
    
    println!("Enter your PHY101 exam score: ");
    let phy_exam : f32 =  input().parse().expect("Error, re-check it"); //phy exam 

    let pyh1_score : f32 =  phy1ca1 + phy1ca2 + p_mark + phy_exam; // total mark 

     // logic of the grade determination
    let pyh1_grade = if pyh1_score >= 70.0 {
        "'A' Excellent!"
    } else if pyh1_score >= 60.0 && pyh1_score <= 69.0 {
        "'B' Very Good"
    } else if pyh1_score >= 50.0 && pyh1_score < 60.0 {
        "'C' Good,"
    } else if pyh1_score >= 40.0 && pyh1_score < 50.0 {
        "'D' Fair"
    } else {
        "'F' You need to work hard!"
    };


//phy107
     println!(" --------PHY107 scores --------");
    println!("Enter your PHY107 1st CA score: ");
    let phy7ca1 : f32 =  input().parse().expect("Error, re-check it"); //phy107 first test 
    
    println!("Enter your PHY107 2nd CA score: ");
    let phy7ca2 : f32 =  input().parse().expect("Error, re-check it"); //phy107 2nd test 

    println!("Enter your PHY107 Participation mark: ");
    let p7_mark : f32 =  input().parse().expect("Error, re-check it"); //phy107 participation mark 
    
    println!("Enter your PHY107 exam score: ");
    let phy7_exam : f32 =  input().parse().expect("Error, re-check it"); //phy exam 

    let pyh7_score : f32 =  phy7ca1 + phy7ca2 + p7_mark + phy7_exam; // total mark 

     // logic of the grade determination
    let pyh7_grade = if pyh7_score >= 70.0 {
        "'A' Excellent!"
    } else if pyh7_score >= 60.0 && pyh7_score <= 69.0 {
        "'B' Very Good"
    } else if pyh7_score >= 50.0 && pyh7_score < 60.0 {
        "'C' Good,"
    } else if pyh7_score >= 40.0 && pyh7_score < 50.0 {
        "'D' Fair"
    } else {
        "'F' You need to work hard!"
    };

//mth101
    println!(" --------MTH101 scores --------");
    println!("Enter your MTH 1st CA score: ");
    let mthca1 : f32 =  input().parse().expect("Error, re-check it"); //mth first test 
    
    println!("Enter your MTH 2nd CA score: ");
    let mthca2 : f32 =  input().parse().expect("Error, re-check it"); //mth 2nd test 

    println!("Enter your MTH Participation mark: ");
    let m_mark : f32 =  input().parse().expect("Error, re-check it"); //mth participation mark 
    
    println!("Enter your MTH exam score: ");
    let mth_exam : f32 =  input().parse().expect("Error, re-check it"); //mth exam 

    let mth_score : f32 =  mthca1 + mthca2 + m_mark + mth_exam; // total mark 

     // logic of the grade determination
    let mth_grade = if mth_score >= 70.0 {
        "'A' Excellent!"
    } else if mth_score >= 60.0 && mth_score <= 69.0 {
        "'B' Very Good"
    } else if mth_score >= 50.0 && mth_score < 60.0 {
        "'C' Good,"
    } else if mth_score >= 40.0 && mth_score < 50.0 {
        "'D' Fair"
    } else {
        "'F' You need to work hard!"
    };

//sta111
    println!(" --------STA111 scores --------");
    println!("Enter your STA111 1st CA score: ");
    let staca1 : f32 =  input().parse().expect("Error, re-check it"); //sta111 first test 
    
    println!("Enter your STA111 2nd CA score: ");
    let staca2 : f32 =  input().parse().expect("Error, re-check it"); //sta 2nd test 

    println!("Enter your STA111 Participation mark: ");
    let s_mark : f32 =  input().parse().expect("Error, re-check it"); //sta participation mark 
    
    println!("Enter your STA111 exam score: ");
    let sta_exam : f32 =  input().parse().expect("Error, re-check it"); //sta exam 

    let sta_score : f32 =  staca1 + staca2 + s_mark + sta_exam;  // total mark 

     // logic of the grade determination
    let sta_grade = if sta_score >= 70.0 {
        "'A' Excellent!"
    } else if sta_score >= 60.0 && sta_score <= 69.0 {
        "'B' Very Good"
    } else if sta_score >= 50.0 && sta_score < 60.0 {
        "'C' Good,"
    } else if sta_score >= 40.0 && sta_score < 50.0 {
        "'D' Fair"
    } else {
        "'F' You need to work hard!"
    };


//Theology 
    println!(" --------THEOLOGY scores --------");
    println!("Enter your Theology 1st CA score: ");
    let ctca1 : f32 =  input().parse().expect("Error, re-check it"); //ct first test 
    
    println!("Enter your Theology 2nd CA score: ");
    let ctca2 : f32 =  input().parse().expect("Error, re-check it"); //ct 2nd test 

    println!("Enter your Theology Participation mark: ");
    let c_mark : f32 =  input().parse().expect("Error, re-check it"); //ct participation mark 
    
    println!("Enter your Theology exam score: ");
    let ct_exam : f32 =  input().parse().expect("Error, re-check it"); //ct exam 

    let ct_score : f32 =  ctca1 + ctca2 + c_mark + ct_exam;  // total mark 

     // logic of the grade determination
    let ct_grade = if ct_score >= 70.0 {
        "'A' Excellent!"
    } else if ct_score >= 60.0 && ct_score <= 69.0 {
        "'B' Very Good"
    } else if ct_score >= 50.0 && ct_score < 60.0 {
        "'C' Good,"
    } else if ct_score >= 40.0 && ct_score < 50.0 {
        "'D' Fair"
    } else {
        "'F' You need to work hard!"
    };

//cos101 
     println!(" --------COS101 scores --------");
    println!("Enter your Computer 1st CA score: ");
    let cosca1 : f32 =  input().parse().expect("Error, re-check it"); //cos first test 
    
    println!("Enter your Computer 2nd CA score: ");
    let cosca2 : f32 =  input().parse().expect("Error, re-check it"); //cos 2nd test 

    println!("Enter your Computer Participation mark: ");
    let cos_mark : f32 =  input().parse().expect("Error, re-check it"); //cos participation mark 
    
    println!("Enter your Computer exam score: ");
    let cos_exam : f32 =  input().parse().expect("Error, re-check it"); //ct exam 

    let cos_score : f32 =  cosca1 + cosca2 + cos_mark + cos_exam;  // total mark 

     // logic of the grade determination
    let cos_grade = if cos_score >= 70.0 {
        "'A' Excellent!"
    } else if cos_score >= 60.0 && cos_score <= 69.0 {
        "'B' Very Good"
    } else if cos_score >= 50.0 && cos_score < 60.0 {
        "'C' Good,"
    } else if cos_score >= 40.0 && cos_score < 50.0 {
        "'D' Fair"
    } else {
        "'F' You need to work hard!"
    };

//software packages 
    println!(" --------SOFTEARE Packages scores --------"); 
    println!("Enter your Software Packages 1st CA score: ");
    let spca1 : f32 =  input().parse().expect("Error, re-check it"); //sp first test 
    
    println!("Enter your Software Packages 2nd CA score: ");
    let spca2 : f32 =  input().parse().expect("Error, re-check it"); //sp 2nd test 

    println!("Enter your Software Packages Participation mark: ");
    let sp_mark : f32 =  input().parse().expect("Error, re-check it"); //sp participation mark 
    
    println!("Enter your Software Packages exam score: ");
    let sp_exam : f32 =  input().parse().expect("Error, re-check it"); //sp exam 

    let sp_score : f32 =  spca1 + spca2 + sp_mark + sp_exam; // total mark 

    // logic of the grade determination
    let sp_grade = if sp_score >= 70.0 {
        "'A' Excellent!"
    } else if sp_score >= 60.0 && sp_score <= 69.0 {
        "'B' Very Good"
    } else if sp_score >= 50.0 && sp_score < 60.0 {
        "'C' Good,"
    } else if sp_score >= 40.0 && sp_score < 50.0 {
        "'D' Fair"
    } else {
        "'F' You need to work hard!"
    };

//Grand total
  let total_score: f32 = pyh1_score + pyh7_score + mth_score + sta_score + ct_score + cos_score + sp_score; 

//grand total grade logic
 let grand_grade = if total_score >= 490.0 {
        "'A' Excellent!"
    } else if total_score >= 420.0 && total_score <= 489.0 {
        "'B' Very Good"
    } else if total_score >= 350.0 && total_score <= 419.0 {
        "'C' Good,"
    } else if total_score >= 280.0 && total_score <= 349.0 {
        "'D' Fair"
    } else {
        "'F' You need to work hard!"
    };

//Demo calculation of GPA
   let gpa: f32 = (total_score / 700.0 ) * 5.0;

    //Displayinng student details with grade 

    println!("=========== Coureses Datails ==============");
    println!("PHY101 score: {}    Grade: {}", pyh1_score, pyh1_grade);
    println!("PHY107 score: {}    Grade: {}", pyh7_score, pyh7_grade);
    println!("MTH101 score: {}    Grade: {}", mth_score, mth_grade);
    println!("STA111 score: {}    Grade: {}", sta_score, sta_grade);
    println!("Theoology score: {}    Grade: {}", ct_score, ct_grade);
    println!("COS101 score: {}    Grade: {}",cos_score, cos_grade);
    println!("Software Packages: {}   Grade: {}",sp_score, sp_grade);

    //Grand details 
    println!("
        ID: {}\n 
        Name: {}\n
        Department: {}\n
        Total Score: {}\n
        Grade : {}\n
        GPA : {:.2}
        ", id, name, department, total_score,grand_grade, gpa);
    println!("=========================================================");
    
}