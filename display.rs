use std::fmt;

// Struct printing: #[derive(Debug)]
// Custom printing (e.g. Display, binary) needs to be implemented

#[derive(Debug)] // For a struct to print with {:?}, must derive Debug
struct ComplexNum {
    real: f64,
    imag: f64,
}

// Implement display for point2d
impl fmt::Display for ComplexNum {
    // Copy-pastable for all display implementations
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Write writes into the first argument (the buffer, f)
        // write! is exactly the same as format! otherwise
        write!(f, "{} + {}i", self.real, self.imag)
        
    }
}

fn main() {
    let point = ComplexNum{
        real: 1.0,
        imag: 2.0
    };
    println!("Debug: {:?}", point);
    println!("Display: {}", point);
}
