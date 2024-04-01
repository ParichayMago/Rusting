struct area(u32, u32);

fn main() {
  let width =10;
  let height = 20;

  let rect = get_area(area(width, height));

  println!("{}", rect)
}

fn get_area(areaa: area) -> u32 {
  return areaa.0 * areaa.1;
}
