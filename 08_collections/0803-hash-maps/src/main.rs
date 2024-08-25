use std::collections::HashMap;

fn hash_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    // map.insert(field_name, field_value);
    // without cloning, this code wouldn't compile because we try to use it
    // after the hashmap has taken ownership of the parameters
    map.insert(field_name.clone(), field_value.clone());

    println!("{field_name} - {field_value}")
}

fn updating_hash_maps() {

    let mut scores = HashMap::new();

    let blue_team = String::from("Blue");

    scores.insert(blue_team.clone(), 10);
    println!("{scores:?}");
    scores.insert(blue_team.clone(), 25);
    println!("{scores:?}");

    scores.entry(blue_team.clone()).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{scores:?}");
}


fn main() {
    println!("Executing hash_ownership");
    hash_ownership();
    println!("Executing updating_hash_maps");
    updating_hash_maps()
}
