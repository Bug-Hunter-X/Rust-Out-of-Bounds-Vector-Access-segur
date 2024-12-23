fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let index = 10;

    match numbers.get(index) {
        Some(value) => println!("Value at index {} is: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    }
}