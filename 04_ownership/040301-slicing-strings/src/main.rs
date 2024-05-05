fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let mut s: String = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error, as clear needs to create a mutable reference in order to truncate it and
    // this would invalidate the reference to word later

    println!("the first word is: {}", word);
    println!("s: {}", s);
    s.push_str(", testing!");
    println!("s: {}", s);
    println!("the final character is: {}", &s[s.len()-1..]);
    // println!("{word}")  // cannot reference here as this would render the s.push_str() invalid
}