// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let vec1 = vec![10, 20, 30, 40];

    println!("Element Index Length: {:?}", vec1.len());

    for element in vec1 {
        println!("Element: {:?}", element);
    }
}
