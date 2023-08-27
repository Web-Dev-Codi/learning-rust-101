// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn return_a_tuple() -> (i32, i32) {
    (1, 5)
}

fn main() {
    // Using the return of return_a_tuple function as x ynd y.
    let (x, y) = return_a_tuple();

    if y < 5 {
        println!("I am less then 5")
    } else if y > 5 {
        println!("I am greater then 5")
    } else {
        println!("I am equal too 5")
    }
}
