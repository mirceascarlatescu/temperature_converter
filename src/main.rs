use std::io;

fn main() {
    println!("Please enter your temperature in Fahrenheit:");

    let mut temperature = String::new();
    
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) =>{ 
                println!("Invalid temperature!");
                return
            }
    };

    const FAHRENHEIT_DIFF : f32 = 32.0;
    let celsius: f32 = ((temperature - FAHRENHEIT_DIFF)/1.8) as f32;
    println!("Your celsius temperature is {:.2}", celsius);
}
