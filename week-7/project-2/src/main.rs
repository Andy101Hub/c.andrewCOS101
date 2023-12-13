use std::io;

fn num_of_siblings() -> i32 {
    let mut input = String::new();
    println!("How many siblings do you have");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Failed to read input");
    n
}
/*
let mut myarr:Vec<String> = Vec::new(); 
myarr.push(name.to_string());

        for val in myarr.iter() {
         println!("{:?}",val );

        }


*/



fn main() {
    let mut myarr:Vec<String> = Vec::new(); 
    let n = num_of_siblings();

    for i in 0..n {
        let mut name = String::new();
        println!("Name of sibling ({}):", i + 1);  // Fixed the indexing issue and added a missing semicolon
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim();

        let namestr = format!("Name of sibling {}: {}",i + 1, name);
        myarr.push(namestr.to_string());

        

        let mut age = String::new();
        println!("Age of sibling ({}):", i + 1);
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let age:i32 = age.trim().parse().expect("Failed to read input");
        let agestr = format!("Age of sibling {}: {}",i + 1, age);
        myarr.push(agestr.to_string());

        if age > 18 {
            let mut status = String::new();
            println!("married or single: ");
            io::stdin().read_line(&mut status).expect("Failed to read input");
            let status = status.trim().to_lowercase();
            let statusstr = format!("Status of sibling {}: {}",i + 1, status);
            myarr.push(statusstr.to_string());

            if status == "single"{
                let mut occupation = String::new();
                println!("student or worker: ");
                io::stdin().read_line(&mut occupation).expect("Failed to read input");
                let occupation = occupation.trim().to_lowercase();
                let occupationstr = format!("Occupation of sibling {}: {}",i + 1, occupation);
                myarr.push(occupationstr.to_string());

                if occupation == "student"{
                    let mut university_name = String::new();
                    println!("Name of university: ");
                    io::stdin().read_line(&mut university_name).expect("Failed to read input");
                    let university_name = university_name.trim();
                    let university_namestr = format!("University of sibling {}: {}",i + 1, university_name);
                    myarr.push(university_namestr.to_string());

                    let mut course_of_study = String::new();
                    println!("Course of study: ");
                    io::stdin().read_line(&mut course_of_study).expect("Failed to read input");
                    let course_of_study = course_of_study.trim();
                    let course_of_studystr = format!("Course of sibling {}: {}",i + 1, course_of_study);
                    myarr.push(course_of_studystr.to_string());
                }

            }
            else if status == "married"{

                let mut offspring_status = String::new();
                println!("Any offring? (yes/no)");
                io::stdin().read_line(&mut offspring_status).expect("Failed to read input");
                let offspring_status = offspring_status.trim();
                let offspring_statusstr = format!("Offspring of sibling {}: {}",i + 1, offspring_status);
                myarr.push(offspring_statusstr.to_string());

                let mut family_city = String::new();
                println!("Family city of residence: ");
                io::stdin().read_line(&mut family_city).expect("Failed to read input");
                let family_city = family_city.trim();
                let family_citystr = format!("Family City of sibling {}: {}",i + 1, family_city);
                myarr.push(family_citystr.to_string());
            }
        }
        else if age <= 18{
            let mut waec_status = String::new();
            println!("Written WAEC? (yes/no)");
            io::stdin().read_line(&mut waec_status).expect("Failed to read input");
            let waec_status = waec_status.trim().to_lowercase();
            let waec_statusstr = format!("WAEC status of sibling {}: {}",i + 1, waec_status);

            myarr.push(waec_statusstr.to_string());

            if waec_status == "yes"{
            let mut schl_name = String::new();
            println!("Name of Secondary School: ");
            io::stdin().read_line(&mut schl_name).expect("Failed to read input");
            let schl_name = schl_name.trim();
            let schl_namestr = format!("School name of sibling {}: {}",i + 1, schl_name);

            myarr.push(schl_namestr.to_string());
        }
            else if waec_status == "no"{
                let mut class_level = String::new();
                
                println!("What is your current class level");
                io::stdin().read_line(&mut class_level).expect("Failed to read input");
                let class_level = class_level.trim();
                let class_levelstr = format!("Class of sibling {}: {}",i + 1, class_level);

                myarr.push(class_levelstr.to_string());
            }


        }





         
    }

         
            for val in myarr.iter() {
         println!("{}",val );



        }
}
