fn main() {
    let x=5;

    let x=x+1;

    {
        let x=x*2;
        println!("Value of x in inside block : {x}");
    }

    println!("Value of x in main block : {x}");

    ///////////

    let spaces="  ";
    let spaces= spaces.len(); // Data type changed with same name with shadowing

    println!("Spaces length : {spaces}");

    let z='😻'; // Char contains more than ASCII ( 4 bytes )
    println!("{z}");
}
