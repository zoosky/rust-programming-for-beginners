// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// * Use an enum for the box color
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Dimensions {
    length: f32,
    width: f32,
    height: f32,
}

// * Use a struct to encapsulate the box characteristics
struct ShoppingBox {
    dimensions: Dimensions,
    color: Color,
    weight: f32,
}

impl ShoppingBox {
    fn new(dimensions: Dimensions, color: Color, weight: f32) -> Self {
        Self {
            dimensions,
            color,
            weight,
        }
    }

    fn print_specification(&self) {
        println!(
            "Dimensions: {} x {} x {}",
            self.dimensions.length, self.dimensions.width, self.dimensions.height
        );
        println!("Color: {:?}", self.color);
        println!("Weight: {}", self.weight);
    }
}
// Notes:
// * Implement functionality on the box struct to create a new box

fn main() {
    // * Implement functionality on the box struct to print the characteristics
    let dim = Dimensions {
        length: 4.0,
        width: 5.0,
        height: 6.0,
    };
    let sbox1 = ShoppingBox {
        dimensions: dim,
        color: Color::Red,
        weight: 34.5,
    };
    let sbox2 = ShoppingBox::new(dim, Color::Green, 12.0);
    sbox1.print_specification();
    sbox2.print_specification();
}
