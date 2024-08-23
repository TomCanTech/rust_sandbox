

fn main() {
    let several_pos =  PartOfSpeech::MultiplePOS(vec![PartOfSpeech::Noun(true), PartOfSpeech::Noun(false)]);

    println!("There are {} nouns of which {} are phrasal.", several_pos.return_multiple()
        .unwrap_or_else(|| vec![])
        .len(),
        several_pos.return_multiple()
        .unwrap_or_else(|| vec![])
        .iter()
        .filter(|pos| match pos.is_phrasal() {
            Ok(phrasal) => phrasal,
            Err(_) => false
        })
        .count()
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

#[derive(Clone)]
enum PartOfSpeech {
    MultiplePOS(Vec<PartOfSpeech>),
    Noun(bool)
}

impl PartOfSpeech {
    fn return_multiple(&self) -> Option<Vec<PartOfSpeech>> {
        match self {
            PartOfSpeech::MultiplePOS(multiple) => Some(multiple.to_vec()),
            _other => None
        }
    }
    fn is_phrasal(&self) -> Result<bool, &str> {
        match self {
            PartOfSpeech::Noun(phrasal) => Ok(*phrasal),
            other => Err("Not a phrasal noun")
        }
    }
}