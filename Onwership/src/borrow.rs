fn borrow() {
  let mut my_strng = String::from("hello_world");
  
}

fn new_borrow(s: &mut String) {
  s.push_str("new world");
}