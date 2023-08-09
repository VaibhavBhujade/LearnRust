use std::io;

fn main() {

    let arr = [1,2,3,4,5];

    let months = ["January","February","March","April","May","June","July","August",
                            "September","October","November","December"];

    let arr : [i64;3] = [200,221,222];

    let arr = [5;3]; // [5,5,5]

    let mut index = String::new();

    println!("Please enter an index (between 0-2 inclusive)");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index : usize = index
        .trim()
        .parse()
        .expect("Enter a valid number");

    let value = arr[index];

    println!("Element at index {index} is {value}");
}
