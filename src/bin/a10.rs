// Topic: Working with expressions
//

// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message() {
    let value: i32 = 55;
    let blean: bool = if value > 100 { true } else { false };

    let print_mes = match blean {
        true => println!("It's Big"),
        false => println!("it's Small"),
    };
}

fn main() {
    print_message();
}
