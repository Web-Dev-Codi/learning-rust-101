// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}
// Using barrowing of the GroceryItem
fn display_quantity(grocery_item: &GroceryItem) {
    println!("Quantity: {:?}", grocery_item.quantity)
}
// Using barrowing of the GroceryItem
fn display_id(grocery_item: &GroceryItem) {
    println!("ID: {:?}", grocery_item.id)
}

fn main() {
    // Creating an instance of the GroceryItem struct
    let meal = GroceryItem { quantity: 5, id: 9 };
    // Using the ampersand to initialize barrowing
    display_quantity(&meal);
    display_id(&meal)
}
