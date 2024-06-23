struct Rect {
    width: u32,
    height: u32,
}

enum Shape {
    
}

impl Rect {
    fn area(&self)-> u32 {
         self.width * self.height 
    }
    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 10,
    };

    println!("{}", rect.area());
    println!("{}", rect.perimeter());
}

enum Result <A, B> {
    Ok(A),
    Err(B)
}