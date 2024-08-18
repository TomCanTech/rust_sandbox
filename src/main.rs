

fn main() {
    let shape = Cuboid{
        width: request_length("width"),
        height: request_length("height")
    };
    println!("The rectangle area is equal to {}", shape.width*shape.height);
}
struct Cuboid{
    width: u32,
    height: u32
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