
pub fn main() {
// Topic: Working with expressions
//
// Requirements:
    let variable = 120;
    let mybool = variable > 100;
    printmybool(mybool);
}
fn printmybool(mybool: bool){
    match mybool {
        true => println!("It's big"),
        false => println!("It's small"),
    }
}

// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print





