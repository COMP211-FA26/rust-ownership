fn borrow_string(s: &String) {
    println!("Borrowed: {}", s);
}

fn own_string(s: String) {
    println!("Cloned: {}", s);
}

fn main() {
    let msg = String::from("Hello");

    borrow_string(&msg);
    println!("After borrowing: {}", msg);

    own_string(msg.clone());
    println!("After cloning: {}", msg);
}
