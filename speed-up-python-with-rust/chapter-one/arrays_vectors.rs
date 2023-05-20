use std::collections::HashMap;

enum Value {
    Str(&'static str),
    Int(i32),
}

fn check_int_above_threshold(
    threshold: i32,
    get_result: Option<&Value>,
) -> Result<bool, &'static str> {
    match get_result {
        Some(inside_value) => match inside_value {
            Value::Str(_) => return Err("str value was supplied it should be int"),
            Value::Int(int_value) => {
                if int_value > &threshold {
                    return Ok(true);
                }
                return Ok(false);
            }
        },
    }
}

fn hashmap_exp() {
    let mut map = HashMap::new();
    map.insert("one", Value::Str("1"));
    map.insert("two", Value::Int(2));

    let result: Option<&Value> = map.get("two");
    let above_threshold: bool = check_int_above_threshold(1, result).unwrap();
    println!("it is {} that threshold is breached", above_threshold);

    let second_result: Option<&Value> = map.get("one");
    let _second_threshold: bool =
        check_int_above_threshold(1, second_result).expect("an error happened");
}

fn loop_vector() {
    let mut str_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("{}", str_vector.len());
    str_vector.push("four");
    for i in str_vector.iter() {
        println!("{}", i)
    }
}

fn main() {
    let array: [i32; 3] = [1, 2, 3];
    for i in array.iter() {
        println!("{}", i);
    }
    loop_vector();
    hashmap_exp();
}
