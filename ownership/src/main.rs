fn main() {
    let s1 = String::from("Hello world");

    let (s2, length) = calculate_length(s1);

    println!("{s2}, {length}",)
}

fn calculate_length(s: String) -> (String, usize) {
    let lenght = s.len();

    (s, lenght)
}
