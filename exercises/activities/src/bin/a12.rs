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

struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}

enum Color {
    Blue,
    Purple,
    Pink,
    Green,
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: i32,
    color: Color,
}

impl ShippingBox {
    fn create_new_box(dimensions: Dimensions, weight: i32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_box_characteristics(&self) {
        println!(
            "The box dimensions of the box is {:?}cm heigth, {:?}cm width, {:?}cm depth. The box weigths {:?} grams.",
            &self.dimensions.height, &self.dimensions.width, &self.dimensions.depth, &self.weight
        );
        match &self.color {
            Color::Blue => println!("The box is blue"),
            Color::Purple => println!("The box is purple"),
            Color::Pink => println!("The box is pink"),
            Color::Green => println!("The box is green"),
        }
    }
}

fn main() {
    let dimensions = Dimensions {
        height: 33.3,
        width: 33.3,
        depth: 33.3,
    };
    let magic_box = ShippingBox::create_new_box(dimensions, 22, Color::Blue);
    ShippingBox::print_box_characteristics(&magic_box);
}
