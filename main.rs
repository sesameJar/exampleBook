use std::fmt;

#[derive(Debug)]
struct Point2D {
    real: f32,
    imag:f32
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{} + {}i", self.real, self.imag)
    } 
}

fn main (){
    let coor = Point2D {
        real : 3.2,
        imag : 78.2
    };

    println!("{}",coor);
    println!("{:?}", coor);
}