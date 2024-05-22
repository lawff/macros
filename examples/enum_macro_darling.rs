use macros::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: i32,
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    let left: Direction = 10.into();
    println!("{:?}, {:?}", up, left);
}

impl DirectionUp {
    fn new(speed: i32) -> Self {
        Self { speed }
    }
}
