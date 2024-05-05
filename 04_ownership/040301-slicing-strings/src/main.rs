fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slices_mutable_immutable() {
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

fn first_word_slice(s: &str) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slices_as_parameters() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_slice(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);
}

fn main() {
    slices_mutable_immutable();
    slices_as_parameters();
}