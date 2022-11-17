fn print_literal(input: String){
    println!("{}", input);
}

fn print_borrow(input: &str) {
    println!("{}", input);
}

fn main() {
    let string_literal = "hello world!";
    print_literal(string_literal.to_string());
    print_borrow(&string_literal);
}