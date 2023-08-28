// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Black,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Black => println!("Black"),
            Color::Red => println!("Red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.7,
        height: 2.1,
        depth: 3.2,
    };

    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    small_box.print();
}
