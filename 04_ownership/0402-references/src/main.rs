fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // some_string.push_str(", world"); // This would throw a compile-time error
                                        // as we can't modify a borrowed value
    s.len()
}