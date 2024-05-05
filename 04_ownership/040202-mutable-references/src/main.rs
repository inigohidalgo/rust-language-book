fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    
    // once a mutable reference is made, we can no longer use prior references.
    // these references are still in scope, but the compiler can see that they aren't being used,
    // so it does not throw an error. as soon as we try to use them, the compiler will throw an error
    // println!("{r1}, {r3}");

}
