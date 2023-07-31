
fn main() {
    //`patterns` in Rust are literal patterns that allow data to be categorized.

    //In C++ there are things like switches. These switches provide something similar to what Rust
    // calls `patterns`. For example I can put an enum in a switch in C++ and match the result.
    // However, patterns in Rust are far more powerful than these switch statements in C++. This
    // chapter explains some of the reasons why.
    all_places_patterns_can_be_used();
    refutability_whether_a_pattern_might_fail_to_match();
    pattern_syntax();
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

fn refutability_whether_a_pattern_might_fail_to_match() {
    //There are two types of patterns in rust.
    // Refutable: These patterns cannot fail to match, `let x = 5` for example will always match.
    // Irrefutable: These patterns can fail to match, `if let Some(x) = var` for example could be
    //  None.
    // `let` and `for` only accept irrefutable patterns.
    // `if let` and `while let` accept refutable patterns as well.

    //The main point of this section is to show that irrefutable and refutable patterns cannot be
    // interchanged. The example is that the line `let Some(x) = var` will not compile because `let`
    // only takes refutable patterns. However, something else interesting is that syntactically it
    // makes a little more sense to me now. When I have a line of `if let Some(x) = var` this is
    // actually setting a variable x, I always thought of it as a comparison, but now I understand
    // why the syntax was set up this way. This actually makes a lot more sense.

    //Using an irrefutable pattern with `if let` will work, but produce an warning.
    // if let x = 5 {
    //     println!("x: {x}");
    // }

}

fn pattern_syntax() {
    //This section is a list of all pattern and the recommendation for when to use each.

    //Matching literals is the most basic way.
    let x = "hello";

    match x {
        "hello" => println!("'hello' found"),
        "world" => println!("'world' found"),
        "rust" => println!("'rust' found"),
        _ => println!("unknown value found"),
    }

    //Matching multiple patterns is another way of accomplishing the same thing.
    match x {
        "hello" | "world" | "rust" => println!("'{x}' found"),
        _ => println!("unknown value found"),
    }

   let y = 'c';

    //Matching ranges is possible. Below ranges are used with chars, but numbers are also possible.
    match y {
        'a'..='d' => println!("a-d"),
        'e'..='h' => println!("e-h"),
        _ => println!("unknown"),
    }

    struct Triangle {
        base: isize,
        height: isize,
    }

    let triangle = Triangle{
        base: 5,
        height: 10,
    };

    //This is destructuring structs. The syntax looks a little odd, but it makes a lot more sense
    // when used with a match statements below. Essentially it is creating two new variables `b` and
    // `h` when are bound to the base and height respectively.
    let Triangle{
        base: b,
        height: h,
    } = triangle;

    println!("base: {b} height: {h}");

    //This is using destructing structs with a match statement. Notice it is possible to only
    // match a single one of the members when necessary. This same method can be used for enums as
    // well.
    match triangle {
        Triangle{ base: 12, height: 15} => println!("12 15 triangle"),
        Triangle{ base: 5, height: 10} => print!("5 10 triangle"),
        Triangle{ base: 8, height } => print!("8 {height} triangle"),
        _ => println!("other triangle"),
    }

    struct Screen {
        size: isize,
        x: isize,
        y: isize,
        t: Triangle,
    }

    let screen = Screen {
        size: 10,
        x: 0,
        y: 0,
        t: triangle,
    };

    //Struct and enum deconstruction can also be used in a nested way. This can avoid a nested match
    // statement.
    match screen {
        Screen{size: 10, x: 0, y: 0, t: Triangle{base, height}} => println!("matching screen found {base} {height}"),
        _ => println!("no matching screen found"),
    }

    //Tuples can also be deconstructed.
    let (x, y) = (4, 5);

    println!("{x} {y}");

    //A value can also be ignored by using the `_` or wildcard operator.
    let (_, y) = (3, 2);

    println!("{x} {y}");

    let x = Some(5);
    let y = Some(7);

    match (x, y) {
        (Some(_), Some(_)) => println!("both x and y and `Some`"),
        _ => println!("either x or y is `None`"),
    }

    //It is also worth noting that starting a variable with `_` will make it so that the compiler
    // ignores it and a warning is not thrown.
    let _a = 5;

    //The remaining parts of a value can be ignored with `..`. `..` can also be used in place of
    // and underscore.
    match screen {
        Screen{size, ..} => println!("size is {size}"),
    }

    //The `..` operator can be used to get the 'middle' values. However, this must be unambiguous
    // the commented out line for example will not compile.
    match (4,5,6,7) {
        (first, .., last) => println!("vals are {first} {last}"),
        // (.., middle, ..) => println!("val is {middle}"),
    }

    let x = Some(5);

    //This if statement that is nested in here is called a `match guard`. It allows for more complex
    // statements than patterns allow.
    match x {
        Some(a) if a % 2 == 1 => println!("Some x is odd"),
        Some(_) => println!("Some x is even"),
        None => println!("None"),
    }

    let triangle = Triangle{
        base: 5,
        height: 10,
    };

    //The `@` operator allows for simultaneously binding a variable and comparing it to a value or
    // range.
    match triangle {
        Triangle{base: b @ 5..=10, height: h} => println!("base: {b} height: {h}"),
        _ => println!("no triangle found"),
    }

}
