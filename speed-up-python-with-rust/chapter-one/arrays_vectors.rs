fn loop_vector () {
    let mut str_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("{}", str_vector.len());
    str_vector.push("four");
    for i in str_vector.iter() {
        println!("{}", i)
    }
}

fn main() {
    let array : [i32; 3] = [1,2,3];
    for i in array.iter() {
        println!("{}", i);
    }
    loop_vector()
}