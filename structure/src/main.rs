struct Person<'a> {
    name: &'a str, // Change to mutable reference to String
    //      ^ lifetime specifer 
    age: i32,
}

fn main() {
    let first_name = "yilmaz"; // Convert string slice to String
    let mut person = Person {
        name: &first_name, // Mutable reference to String
        age: 32,
    };

    {
        let last_name = String::from("bingol");
        person.name = &last_name; // Assigning a new String to the mutable reference
        println!("{}", person.name)
    }
    // String -> heap
    // str -> Stack
}
