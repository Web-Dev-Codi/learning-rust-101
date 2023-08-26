// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    let number: i32 = 9;

    if number > 5 {
        println!("Number is greater then 5")
    } else if number < 5 {
        println!("Number is less then {}", number)
    } else if number == 5 {
        println!("Number is equal {}", number)
    }
}

// I completed the task but neglected to use only an else block at the end. Code is working
