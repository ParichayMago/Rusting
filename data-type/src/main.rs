fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -6 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let heart_eyed_cat = '😻';

    add(2,3);

    println!("{heart_eyed_cat}")
}

fn add(a:u32, b:u32) {
    let x:u32 = a + b;

    println!("{x}")
}
