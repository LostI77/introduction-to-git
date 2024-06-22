fn main() {
    println!("Other test file example");

    let string_variable: &str = do_something_return_fn();

    println!("my string variable: {}", string_variable)
}

fn do_something_return_fn() -> &'static str {
    return "Este es un string";
}
