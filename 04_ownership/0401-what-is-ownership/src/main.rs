fn main() {
    let x = 5;                      // x comes into scope
    println!("before call: {x}"); 
    makes_copy(x);                  // x would move into the function,
    println!("after call: {x}")     // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let s1 = String::from("hello");  // s1 comes into scope
    println!("before call s1: {s1}");
    takes_ownership(s1);             // s1's value moves into the function...
    // println!("after call: {s1}"); // ... and so is no longer valid here

    let s2 = gives_ownership();
   
    println!("s2: {s2}")
    let s3 = takes_and_gives_back(s2)
    println!("s3: {s3}")
    // println!("s2: {s2}") // s2 was moved to s3
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("takes ownership: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("call: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}