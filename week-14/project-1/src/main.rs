use::std::io;
use::std::io::Read;

fn administrator(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn project_manager(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents_1 = String::new();
    file.read_to_string(&mut contents_1).unwrap();
    print!("{}", contents_1);
}

fn employee(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents_2 = String::new();
    file.read_to_string(&mut contents_2).unwrap();
    print!("{}", contents_2);
}

fn customer(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents_3 = String::new();
    file.read_to_string(&mut contents_3).unwrap();
    print!("{}", contents_3);
}

fn vendor(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents_4 = String::new();
    file.read_to_string(&mut contents_4).unwrap();
    print!("{}", contents_4);
}


fn main() {
    loop{
        println!("Welcome to Globacom Database, please input position: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let input1 = input1.trim().to_lowercase();

        if input1 == "administrator"{
            administrator();
            break;
        }

        else if input1 == "project_manager"{
            project_manager();
            break;
        }

        else if input1 == "employee"{
            employee();
            break;
        }

        else if input1 == "customer"{
            customer();
            break;
        }

        else if input1 == "vendor"{
            vendor();
            break;
        }


        }
}
