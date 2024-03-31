fn main() {
    let mut s: String = String::from("hello");

    let p = &mut s;
    let q = &mut s;
    change(&mut s);
    change(&mut s);

    let n = calculate_length(&s);

    println!("{n} {s}");

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(str: &String) -> usize {
    return str.len();
}
