use std::io;

fn main() {

    const INVALID_INPUT_MESSAGE: &str = "You gotta enter the specified choices only";
    println!("Welcome to Temp Convertor");
    println!("Choose one");
    println!("1) Fahrenheit to Celsius");
    println!("2) Celsius to Fahrenheit ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failure to read the line");

    

    let choice = match choice.trim().parse::<i8>() {
        Ok(ch) => ch,
        Err(_) => panic!("{}", INVALID_INPUT_MESSAGE),
    };



    if choice != 1 && choice != 2 {
        println!("your choice is: {}", choice);
        panic!("{}",INVALID_INPUT_MESSAGE);
    }

    println!("Enter the value to convert");

    let mut value = String::new();

    io::stdin().read_line(& mut value).expect("Failed to read the line");

    let value = match value.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => panic!("{}", INVALID_INPUT_MESSAGE),
    };

    let result:f64;

    if choice == 1 {
        result = f_to_c(value)
    } else {
        result = c_to_f(value)
    }

    println!("The result is: {}", result)


    
}


fn f_to_c (feh:f64) -> f64 {
    (feh - 32.0) * 5.0/9.0
} 


fn c_to_f (c:f64) -> f64 {
    (c *  9.0/5.0 )  + 32.0
} 