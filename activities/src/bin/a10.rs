// Topic: Working with expressions
//
// Requirements:
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

fn main() {
    let mut is_small = false;
    let num = 100;

    if num <= 100 {
        is_small = true;
    } else {
        is_small = false;
    }

    match is_small {
        true => print!("its small"),
        false => print!("its big"),
    };
}
