// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Cherry,
    Melon,
    Apple,
}

struct Beverage {
    ounchs: f64,
    flavor: Flavors,
}

fn drink(f: Beverage) {
    match f.flavor {
        Flavors::Cherry => println!("Flavor: Cherry"),
        Flavors::Melon => println!("Flavor: Melon"),
        Flavors::Apple => println!("Flavor: Apple"),
    }

    println!("oz: {:?}", f.ounchs);
}

fn main() {
    let cherry_drink = Beverage {
        ounchs: 12.0,
        flavor: Flavors::Cherry,
    };

    let apple_drink = Beverage {
        ounchs: 16.0,
        flavor: Flavors::Apple,
    };
    let melon_drink = Beverage {
        ounchs: 8.0,
        flavor: Flavors::Melon,
    };
    drink(cherry_drink);
    drink(apple_drink);
    drink(melon_drink);
}
