use std::io;
pub fn mainfun(){

    println!("-- Simple Adder (Safe Version) --");
    let x = get_number("X");
    let y = get_number("Y");

    println!("The sum of {} and {} is {}", x, y, x + y);
}
fn get_number(label: &str) -> i32 {
    loop {
        println!("Enter the value for {}:", label);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => return num, // Success! Return the number and exit the loop
            Err(_) => {
                println!("⚠️ Error: '{}' is not a valid number. Please try again.", input.trim());
                continue; // Stay in the loop and ask again
            }
        }
    }
}