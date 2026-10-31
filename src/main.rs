fn main() {
    let name = "Abraham";
    welcome_message(name);
}

fn welcome_message(name: &str) {
    println!("Welcome to Rust-World!");
    println!("Nice to meet you, {}!", name);
}
