fn main() {
    println!("Hello, world!");
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    // pick the first item: 
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item: 
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent: 
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("there is no fruit! :("),
        }
    }
}
