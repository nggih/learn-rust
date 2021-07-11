fn main() {
    println!("Hello, world!");
    println!("The first letter of the English alphabet is {} and the last letter is {}.", 'A', 'Z');
    let a_number;
    let a_word = "Ten";
    a_number = 10;
    println!("the number is {}.", a_number);
    println!("the word is {}.", a_word);


    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;
    println!("The number is {}.", shadow_num);

    let number: u32 = 14;
    println!("The number is {}.", number);

    let is_bigger = 1 > 4;
    println!("Is 1 > 4 ? {}", is_bigger);

    let tuple_e = ('e', 5i32, true);

    println!("is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);

    if 1 == 2 {
        println!("True, the numbers are equal."); //
    } else {
        println!("False, the numbers are not equal.");
    }

    // if as expression
    let formal = true;
    let greeting = if formal {
        "Good day to you."
    } else {
        "Hey!"
    };
    println!("{}", greeting);

    // Classic struct with named fields 
    struct Student { name: String, level: u8, remote: bool }

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    // Unit struct 
    struct Unit;
    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
         user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
         user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}
