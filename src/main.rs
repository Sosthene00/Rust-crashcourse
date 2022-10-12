
struct Square<T> 
{
    side: T,
}

struct Triangle
{
    side1 : f64,
    side2 : f64,
    side3 : f64
}

impl Square<f64> {
    fn new( t: f64) -> Self {
        Square { side : t}
    }
}

impl Square<u32> {
    fn new( t: u32) -> Self {
        Square { side : t}
    }

}

impl Square<String> {
    fn new( t: &str) -> Self {
        let float : f64;
        float = t.parse().unwrap();
        Square { side : (float).to_string(),
    }
}
}

impl Triangle {
    fn new(c1:f64,c2:f64) -> Self {
        Triangle { side1 : c1, side2 : c2, side3 : (c1*c1 + c2*c2).sqrt() }
    }
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Square<f64> {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Area for Square<u32> {
    fn area(&self) -> f64 {
        self.side as f64 * self.side as f64
    }
}


impl Area for Square<String> {
    fn area(&self) -> f64 {
        let tamp : f64 = self.side.parse().unwrap();
        tamp*tamp
    }
}


fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.side);
    println!("square_float area is {}", square_float.side);
    println!("square_string area is {}", square_string.side);

    
    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    /*
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
    */
}
