// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

pub fn main() {
    let mut myint = 1;
    let mut i = 0;
    loop {
        let mut mynewint = myint + i;
        println!("{:?}", mynewint);
        i = i+1;
        if mynewint == 4 {
            break;
        }
    }

}
