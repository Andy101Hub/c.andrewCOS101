
use std::io;

fn main() {

    let mut candidate_number = String::new();
    let mut class_rep = String::new();
    let mut level = String::new();
    let mut cgpa = String::new();
    let mut candidate_name = String::new();
    let mut candidate_email = String::new();
    let mut candidate_department = String::new();
    let mut candidate_state = String::new();
    


    println!("Hello candidate, what's your number? ");
    io::stdin().read_line(&mut candidate_number).expect("Failed to read input");
    let candidate_number:i32 = candidate_number.trim().parse().expect("Failed to read input");

    if candidate_number > 150{
        println!("Only the first 150 can vote");
    }

    else{

        println!("Are you a current class rep? ");
        io::stdin().read_line(&mut class_rep).expect("Failed to read input");
        let class_rep = class_rep.trim().to_lowercase();
      
        if class_rep == "no"{
            println!("You are not eligible to vote" );
        }
    

        else {
            println!("What level are you in? ");
            io::stdin().read_line(&mut level).expect("Failed to read input");
            let level:i32 = level.trim().parse().expect("Failed to read input");

            if level == 100 {
                println!("You are not eligible to vote");
                return;
            }

            else {
                println!("What is your CGPA? ");
                io::stdin().read_line(&mut cgpa).expect("Failed to read input");
                let cgpa = cgpa.trim();

                if cgpa < &4.0.to_string(){
                    println!("You are not eligible to vote", );
                    return;
                }

                else{
                    println!("Fill the form below");
                }
                
               
    }          

                println!("Name: " );
                io::stdin().read_line(&mut candidate_name).expect("Failed to read input");
                let candidate_name = candidate_name.trim();

                println!();

                 println!("Email: " );
                io::stdin().read_line(&mut candidate_email).expect("Failed to read input");
                let candidate_email = candidate_email.trim();

                println!();

                 println!("Department: " );
                io::stdin().read_line(&mut candidate_department).expect("Failed to read input");
                let candidate_department = candidate_department.trim();

                println!();

                 println!("State of origin: " );
                io::stdin().read_line(&mut candidate_state).expect("Failed to read input");
                let candidate_state = candidate_state.trim();

                println!();


                println!("YOU CAN VOTE");
            }

        }
    



    
}
