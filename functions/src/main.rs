fn main() {
    println!("Hello, world!");

    another_function("Hello");

    let five = get_number();

    println!("The expression returned : {five}");

    let x = plus_one(5);

    println!("The plus one of 5 is {x}");
}

fn another_function(str: &str) {
    println!("{str} Another function");
}

// Statement : let y=4;
// Expressions ( returns something ) : let y = { let x=4; x+2 }
// Note no semicolon for x+2 the return value

fn get_number() -> i32 {
    5
}

fn plus_one(x:i32) -> i32 {
    x+1
}