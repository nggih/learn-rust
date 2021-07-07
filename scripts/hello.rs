fn main() {
    // this is a comment
    /* this is also a comment 

    macro is expanded into abstract syntax trees, it looks like a functions with a bang !. 
    
    */
    println!("Hello World!"); //println is a macro that prints text to the console
    println!("I'm a Rustacean!");
    let x = 5 + /* 90 + */ 5;
    println!("Is x 10 or 100? x = {}", x);


    println!("{} days", 31);

    // Optional pattenrs. Positional arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject}{verb} {object}", 
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jump over");
    
    // Special formatting can be specified after a ':'.
    println!("{} of {:b} people know binary, the other half doesn't", 1,2);

    // You can right-align text with a specified width. This will output
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001". 
    println!("{number:>0width$}", number=1, width=6); 

    // rust checks to make sure the correct number of arguments are used. 
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Create a structure named 'Structure' which contains an 'i32'. 
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated handling. 
    // This will not work. 
    // println!("This struct `{}` won't print...", Structure(3));

    println!("{number:>width$}", number="test", width=6);

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);


}