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
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blue => println!("Color is Blue"),
            Color::Purple => println!("Color is Purple"),
            Color::Pink => println!("Color is Pink"),
        }
    }
}

impl Dimensions {
    fn print(&self) {
        println!("Height: {:?}", self.height);
        println!("Width: {:?}", self.width);
        println!("Depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    color: Color,
    weight: i32,
    
}

impl ShippingBox {
    fn new(dimensions: Dimensions, color: Color, weight: i32) -> Self {
        Self {
            dimensions,
            color,
            weight,
        }
    }

    fn print(&self) {
        self.dimensions.print();
        self.color.print();
        println!("Weight is: {:?}", self.weight);
    }
}

fn main() {
    let dimensions = Dimensions {
        height: 33.3,
        width: 33.3,
        depth: 33.3,
    };
    let magic_box = ShippingBox::new(dimensions, Color::Pink, 22);

    magic_box.print();
}
