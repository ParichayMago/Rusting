struct area(u32, u32);

fn main() {
    let rect = get_area(area(10, 20));

  println!("{}", rect)
}

fn get_area(areaa: area) -> u32 {
  return areaa.0 * areaa.1;
}
