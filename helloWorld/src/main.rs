fn main() {
    // All values in rust are initially immutable, mut keyword is required.
    let mut x = 45;
    println!("{} + {} = {}",x+x,x+2, x+x+x+2);
    x = 20;
    println!("{} + {} = {}",x+x,x+2, x+x+x+2);
}
