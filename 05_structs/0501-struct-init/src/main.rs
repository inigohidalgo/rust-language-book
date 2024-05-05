struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn instantiating_directly() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // the entire struct must be set as mutable
    // to be able to change any of its fields
    // user1.email = String::from("someoneelse@mail.com"):
    println!("user1 email: {}", user1.email);

    let mut user2 = User {
        active: true,
        username: String::from("anotherusername456"),
        email: String::from("someoneaa@whatever.com"),
        sign_in_count: 3,
    };
    user2.username = String::from("anotherusername789");
    println!("user2 username: {}", user2.username);
}

fn init_functions() {
    fn build_user(email: String, user: String) -> User {
        User {
            active: true,
            username: user,
            email, // field init shorthand, since the parameter and field have the same name
            sign_in_count: 1,
        }
    }
    build_user(String::from("what"), String::from("ever"));
}

fn struct_update_syntax() {
    let user1 = User {
        active: true,
        username: String::from("whocares"),
        email: String::from("someone@there.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("anotherusername"),
        ..user1 // expand rest of fields
    };

    println!("user1: {}", user1.username);
    println!("user2: {}", user2.username);
}

fn struct_derived_traits() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: dbg!(20 / 2),
    };
    println!("rect1 is {:#?}", rect1);
}

fn main() {
    instantiating_directly();
    init_functions();
    struct_update_syntax();
    struct_derived_traits();
}
