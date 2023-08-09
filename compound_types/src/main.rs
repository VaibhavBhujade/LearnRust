fn main() {

    // Tuples : Aggregation of multiple data types
    let x : (i32,f32,u8) = (33,1.423,1);

    let (a,b,c) = x; // Destructuring

    println!("First element : {a} , Second element : {b} , Third element : {c} ");

    let d = x.0;
    let e = x.1;
    let f = x.2;

    println!("First element : {d} , Second element : {e} , Third element : {f} "); // Direct access

    //
}
