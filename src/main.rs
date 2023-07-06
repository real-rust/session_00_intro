fn main() {
    let str_1 = "Hello";
    let str_2 = "World";
    let str_3 = "from Rust!";

    // string interpolation using println!()
    println!("{} {} {}", str_1, str_2, str_3);

    // string interpolation using format!()
    let first_name = "Mir Hassan";
    let last_name = "Moosavi";

    let full_name = format!("{} {} is a Rust programmer!", first_name, last_name);
    println!("{}", full_name);

}

// compile, build and run!
// rustc main.rc