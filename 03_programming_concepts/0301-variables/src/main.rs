fn main() {
    const AA: (char, &str, i32, i32, i32) = ('a', "vv", 3, 4, 5);

    let element = AA.4;

    println!("The value of the element : {element}");
}