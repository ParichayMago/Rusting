fn main() {
  let s = String::from("hello world");
  let word = first_word(&s); // word will get the value 5

  println!("{}", word)
}


fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return i;
      }
  }

  s.len()
}

