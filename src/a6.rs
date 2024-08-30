// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

pub fn main() {
    let mut mycounter = 5;
    println!("{:?}", mycounter);
    while mycounter > 1{
        mycounter = mycounter - 1;
        println!("{:?}", mycounter);
    }
    println!("Done!")
}
