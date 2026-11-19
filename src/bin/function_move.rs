fn consume(s: String) {
    println!("Inside function: {}", s);
}

fn main() {
    let msg = String::from("Hello");

    consume(msg);

    println!("Outside function: {}", msg);
}