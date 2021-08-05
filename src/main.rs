use std::collections::HashMap;

fn print(input_string: String) {
    println!("{}", input_string);
}

fn print_borrow(input_string: &str) {
    // Borrowing it using the & operator
    println!("{}", input_string);
}

fn strings() {
    println!("Hello, world!");
    let hello = String::from("Hello, Armen!"); // String object
    print(hello);

    let test_string = "Hi!"; // String literal
    print(test_string.to_string());

    let test_string2 = &"Hello, World!"; // Borrowing it with & operator
    print_borrow(test_string2);
}

fn vectors() {
    let int_array: [i32; 3] = [1, 2, 3];
    for i in int_array.iter() {
        println!("{}", i);
    }

    // NOTE that hash maps are initialized with HashMap::new()
    let str_vector: Vec<&str> = vec!["one", "two", "three"];

    for i in str_vector.iter() {
        println!("{}", i);
    }

    let second_int_array: [i32; 3] = [1, 2, 3];

    let two = second_int_array[1];
    print(two.to_string());
    // Cannot be modified because it is not mutable
    // str_vector.push("four");
    let mut str_vector_mut: Vec<&str> = vec!["one", "two", "three"];
    str_vector_mut.push("four")
}

fn hash_maps() {
    let mut general_map: HashMap<&str, i8> = HashMap::new();
    general_map.insert("test", 25);
    // Here, we can see that the get method does not actually return an i8 type, despite us
    // inserting an i8 type into the hash map. It's returning an Option enum instead.
    // This is because the get method could fail. We could pass in a key that does not exist.
    // Therefore, we have to unwrap the option to get the value we're aiming to get
    // let outcome: i8 = general_map.get("test");
    let outcome: Option<&i8> = general_map.get("test");
    // Directly unwrapping the result can result in an error being raised. Because Optional is either
    // Some or None, we can exploit Rust's match statement to handle the outcome
    println!("{}", outcome.unwrap());
    match general_map.get("test") {
        None => println!("it failed"),
        Some(result) => println!("Here is the result: {}", result),
    }
}

// Note that there is no return keyword. This is because the function returns
// the final expression in the function when there is no semicolon at the end of the expression.
// --
// the 'static notation (lifetime notation) is telling the compiler that the
// error string will stay around for the entire runtime of the program.
fn error_check(check: bool) -> Result<i8, &'static str> {
    if check == true {
        Err("this is an error")
    } else {
        Ok(1)
    }
}

fn return_values() {
    // In here we unwrap but below we don't
    // let result: i8 = error_check(false).unwrap();
    let result: Result<i8, &'static str> = error_check(true);

    match result {
        Ok(x) => println!("it's a result of: {}", x),
        Err(x) => println!("{}", x),
    }
}
fn main() {
    strings();
    vectors();
    hash_maps();
    return_values();
}
