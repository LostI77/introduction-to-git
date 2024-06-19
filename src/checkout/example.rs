fn main() {
    let something: u32 = return_something();

    println!("{}", something)
}

fn return_something() -> u32 {
    return 8;
}
