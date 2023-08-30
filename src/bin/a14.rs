// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

// Barrowing name and fav_color here to use in our if expression
fn print_info(name: &str, fav_color: &str) {
    println!("Name: {:?} Favorite Color: {:?}", name, fav_color);
}

fn main() {
    let persons = vec![
        Person {
            age: 1,
            name: "Astra".to_owned(), //  using to_owned() for ownership
            fav_color: "Red".to_owned(),
        },
        Person {
            age: 40,
            name: "Brian".to_owned(),
            fav_color: "Black".to_owned(),
        },
        Person {
            age: 35,
            name: "Gülden".to_owned(),
            fav_color: "Purple".to_owned(),
        },
        Person {
            age: 1,
            name: "Neby".to_owned(),
            fav_color: "Green".to_owned(),
        },
    ];

    for i in persons {
        if i.age <= 10 {
            // Barrowed name AND fav_color here
            print_info(&i.name, &i.fav_color);
            println!("{:?} you are a young rugrat", i.name);
        }
    }
}
