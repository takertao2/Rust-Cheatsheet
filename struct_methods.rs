#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// Implementation block: contains methods for specified struct
impl Point {

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    // Top do an operation on self, we pass a reference to seld
    fn magnitude(&self) -> f64 {
        let Point { x: x1, y: y1 } = self;
        (x1 * x1 + y1 * y1).sqrt()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

}

fn main() {
    // Call a struct (or enum) method using Struct::method() syntax
    let mut p = Point::new(3.0, 3.0);
    println!("Point P: {:?}", p);
    // P is already a Point, so self is implictly passed
    let mut m = p.magnitude();
    println!("Magnitude of P: {:.3?}", m);
    p.translate(0.0, 1.0);
    println!("Point P translated y+1: {:?}", p);
    m = p.magnitude();
    println!("Magnitude of P after translation: {:?}", m);
}