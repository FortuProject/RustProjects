// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

pub fn main() {
    println!("running a2.rs");
    let a = 1;
    let b = 4;
    println!("a is {:?} b is {:?}", a, b);
    let sumab = a+b;
    println!("The resulting sum of a and b is {:?}", sumab);
    println!("a2.rs is complete");

    println!("with 3 functions attempt");
    let result = sum(2,2);
    display_results(result);
    println!("with 3 functions Pass");
}

pub fn sum(a: i32, b: i32) -> i32{
    a+b
}

pub fn display_results(result: i32){

    println!("{:?}", result);
}