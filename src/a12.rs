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

pub fn main() {
    let small_dimensions = Dimensions{
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box = ShippingBox::new(5.0, BoxColor::white, small_dimensions);
    small_box.print();
}

impl BoxColor{
    fn print(&self) {
        match self{
            BoxColor::black => println!("black"),
            BoxColor::white => println!("white"),
        }

    }

}

struct Dimensions{
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions{
    fn print(&self){
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);

    }

}

impl ShippingBox {
    fn new(weight: f64, color: BoxColor, dimensions: Dimensions) -> Self{
        Self{
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

struct ShippingBox{
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor,
}

enum BoxColor{
    white,
    black,
}