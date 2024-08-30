// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

pub fn main() {
    let someint = 3;
    match someint {
        1 => println!("It's 1"),
        2 => println!("It's 2"),
        3 => println!("It's 3"),
        _ => println!("It's something else"),
    }

}
