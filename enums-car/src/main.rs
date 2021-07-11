struct Car {
    color: String,
    transmission: Transmission, 
    convertible: bool, 
    mileage: u32,
}
#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car = Car{color, transmission, convertible, mileage:0u32};
    return car;
}

fn main() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    

    // Generic type syntax
    // Declare vector
    let three_nums = vec![15, 6, 36];
    println!("Initial vector: {:?}", three_nums);

    // Declare vector, first value = "0", length = 5
    let zeroes = vec![0; 5];
    // Array signature is defined as [T; size]; 
    // T: the data type for all elements in the array
    // size: nonnegative integer that represents the array length
    println!("Zeroes: {:?}", zeroes);

    // mutable vector 
    let mut fruit = Vec::new();

    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert("Ancient Roman History".to_string(), "Very accurate.".to_string());
    reviews.insert("Cooking with Rhubarb".to_string(), "Sweet recipes.".to_string());
    reviews.insert("Programming in Rust".to_string(), "Great examples.".to_string());

    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));
}