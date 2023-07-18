fn main() {
    //`patterns` in Rust are literal patterns that allow data to be categorized.

    //In C++ there are things like switches. These switches provide something similar to what Rust
    // calls `patterns`. For example I can put an enum in a switch in C++ and match the result.
    // However, patterns in Rust are far more powerful than these switch statements in C++. This
    // chapter explains some of the reasons why.
    all_places_patterns_can_be_used();
}

fn all_places_patterns_can_be_used() {

    //The `match` keyword can be used to match patterns, but it must be exhaustive. Exhaustiveness
    // can be a good thing in a lot of cases. It will alert me to all cases that need updated if
    // say a new enum value was added.
    let my_val = Some(5);
    match my_val {
        Some(x) => println!("number: {x}"),
        None => println!("None"),
    }

    //The `if let` syntax can be used and not only does it not need to be exhaustive, but it doesn't
    // need to reference the same variable or even the same type of variable.
    let age: Result<isize, String> = Ok(5);
    if let None = my_val {
        println!("my_val was None");
    } else if let Ok(age) = age {
        //Note the shadowed variable name here. This is also possible.
        if age > 20 {
            println!("age > 20");
        } else {
            println!("age <= 20");
        }
    } else {
        println!("No conditions reached");
    }

    let mut name = String::from("name");

    //There is also a `while let` loop. Note that this way of setting up a loop is different than a
    // for loop. The below while loop will stop when name.pop() returns `None`.
    while let Some(c) = name.pop() {
        println!("c: {c}");
    }

    let name = String::from("name");

    //The `for` loop also can to pattern matching. This is an example of splitting the results of
    // results of the String.
    for (i, c) in name.chars().enumerate() {
        println!("i: {i} c: {c}");
    }

    //The `let` keyword itself is also a pattern. The syntax of let actually looks like this
    // let PATTERN = EXPRESSION;
    // This is why things like the below statement are valid.
    let (x, y, z) = ('a', 'b', 'c');

    println!("x: {x} y: {y} z: {z}");

    //Function parameters can also use pattern matching.
    fn printing_stuff(&(a, b): &(char, char)) {
        println!("printing_stuff({a}, {b})");
    }

    let stuff = ('a', 'b');
    printing_stuff(&stuff);
}