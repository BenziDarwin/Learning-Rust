use std::ops::Range;

fn main() {
    let numbers: Range<i32> = 1..11;
    for i in numbers {
        println!("{}",i);
    }

    let animals: Vec<&str> = vec!["cat", "dog", "mouse"];
    for (index, a) in animals.iter().enumerate() {
        println!("The animal is {}",animals[index])
    }
}
