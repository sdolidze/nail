use std::collections::HashMap;

#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    age: i32,
}

pub fn run() {
    let people = HashMap::from([
        (
            0,
            Person {
                id: 0,
                name: String::from("Sandro"),
                age: 28,
            },
        ),
        (
            1,
            Person {
                id: 1,
                name: String::from("Mari"),
                age: 30,
            },
        ),
    ]);

    for Person { id, name, age } in people.values() {
        println!("id: {id} name: {name}, age: {age}")
    }
}
