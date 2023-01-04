const MAX_NO: u8 = 2;// This is how constants are defined in Rust.

enum Direction {
    Up,
    Down,
    Left,
    Right
}
fn main() {
    let player : Direction = Direction::Down;
    match player {
        Direction::Up => println!("We are going up!"),
        Direction::Down => println!("We are going down!"),
        Direction::Left => println!("We are going left!"),
        Direction::Right => println!("We are going right!")
    }  
    for i in 0..MAX_NO {
        println!("It is finished!");
    }
}