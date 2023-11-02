use std::io;
fn main() 
{
    let mut distance_miles:f32 = 80.0;
    let miles_to_km:f32 = 0.621371;
    let distance_km = distance_miles/miles_to_km;

    println!("Distance in kilometers is {}km",distance_km); 

    let mut time:f32 = 2.0;
    let mut speed = distance_km/time;

    println!("The speed of the first car is {}km/hr",speed);

    distance_miles = 120.0;
    let distance_km = distance_miles/miles_to_km;

    println!("Distance of the 2nd kilometer is {}km",distance_km );

    time = 4.0;
    speed = distance_km/time;

    println!("The speed of the second car is {}km/hr",speed);

    println!("");

    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Now give me a value for Distance in miles:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    let distance_km = a/miles_to_km;
    println!("This is your value in kilometer {}km",distance_km);

    println!("");

    println!("Good job! now give me a value for time:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    let speed = distance_km/b;

    println!("The speed of your vehicle is {}km/hr",speed);
}