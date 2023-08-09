use std::io;

fn main() {

    println!("Choose conversion 1.Celsius to Fahrenheit 2. Fahrenheit to Celisus");

    let mut choice=String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Unable to read line");

    let choice:i32=choice
        .trim()
        .parse()
        .expect("Please enter a valid number");

    if choice==1 {
        println!("Enter value in Celsius");
        let mut celsius= String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Unable to read line");

        let celsius : f64 = celsius
            .trim()
            .parse()
            .expect("Please enter a valid number");

        let fahrenheit = (9.0/5.0)*celsius + 32.0;

        println!("Fahrenheit equivalent of {celsius}C is {fahrenheit}F");
    } else if choice==2 {
        println!("Enter value in Fahrenheit");
        let mut fahrenheit= String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Unable to read line");

        let fahrenheit : f64 = fahrenheit
            .trim()
            .parse()
            .expect("Please enter a valid number");

        let celsius = (fahrenheit - 32.0)*(5.0/9.0);

        println!("Celsius equivalent of {fahrenheit}F is {celsius}C");
    } else {
        println!("Invalid choice");
    }
}
