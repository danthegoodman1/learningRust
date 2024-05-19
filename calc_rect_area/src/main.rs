#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        dbg!(self.width * self.height)
    }

    fn square(size: u32) -> Rect {
        Rect { width: size, height: size }
    }
}

fn main() {
    let rect = Rect{
        height: 10,
        width: 10
    };
    println!("The area of the rectangle {:#?} is {}", rect, rect.area());
    dbg!(&rect);
    let sq = Rect::square(5);
    dbg!(sq);
}
