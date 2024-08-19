

fn main() {
    let shape = Cuboid{
        width: request_length("width"),
        height: request_length("height")
    };
    let shape_variation = CuboidVar::return_variation(&shape);

    println!("The shape's variation is a {}.", 
        match shape_variation {
            CuboidVar::Square => "square",
            CuboidVar::Rectangle => "rectangle" 
        }
    );
}
struct Cuboid{
    width: u32,
    height: u32
}

impl Cuboid{
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: Cuboid) -> bool {
        (self.width > other.width && self.height > other.height)
        || (self.width > other.height && self.height > other.width)
    }
    fn square(length: u32) -> Self {
        Self {
            width: length,
            height: length
        }
    }
}


fn request_length(side: &str) -> u32 {
    let mut side_str = String::new();
    let side_val: u32;
    loop {
        println!("What is the {side}");
        std::io::stdin()
        .read_line(&mut side_str)
        .expect("Failed to read input");

        side_val = match side_str.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
            side_str = String::new();
            continue},
        };

        break
    }
    side_val
}

enum CuboidVar {
    Square,
    Rectangle
}

impl CuboidVar {
    fn return_variation(shape: &Cuboid) -> Self {
        if shape.width == shape.height {
            return Self::Square
        } else {
            return Self::Rectangle
        }
    }
}