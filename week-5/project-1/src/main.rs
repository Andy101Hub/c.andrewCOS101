
use std::io;

fn main() {
    println!("Enter a value for a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut a:f32 = input1.trim().parse().expect("Failed to read input");

    println!("Enter a value for b: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let mut b:f32 = input2.trim().parse().expect("Failed to read input");

    println!("Enter a value for c: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let mut c:f32 = input3.trim().parse().expect("Failed to read input");

    let d:f32 = 4.0;
    let calc1:f32 = b.powf(2.0); 
    let calc2:f32 = d * a * c;
    let calc3:f32 = calc1 - calc2;
    let calc4:f32 = 2.0 * a;
    let calc5:f32 = -b +calc3.sqrt();
    let calc6:f32 = -b - calc3;
    let calc7:f32 = calc5 / calc4;
    let calc8:f32 = calc6 / calc4;
    

    println!("The roots of your equation are: {},{}", calc7,calc8);


    let x = b.powf(2.0) - 4.0 * a * c;
    let x1 = b.powf(2.0) - 4.0 * a * c;
    let x2 = b.powf(2.0) - 4.0 * a * c;

    if x > 0.0 {
        println!("There are two distinct real roonts");
    }
    else if x1 == 0.0 {
        println!("One repeated real root");
    }
    else if x2 < 0.0 {
        println!("No real roots");
    }
    else {
        println!("The equation has no roots");
    }

}

    
    













    


    

