pub trait Surface<T> {
    fn area(&self) -> T;
}

pub trait Volume<T>{
    fn volume(&self) -> T;
}

struct Square<T>{
    x: T
}

impl Surface<f64> for Square<u32>{
    fn area(&self) -> f64{ 
        (self.x*self.x) as f64
    }
}

impl Surface<f64> for Square<f64>{
    fn area(&self) -> f64{ 
        (self.x*self.x) as f64
    }
}

impl Surface<f64> for Square<String>{
    fn area(&self) -> f64{ 
        self.x.parse::<f64>().unwrap()*self.x.parse::<f64>().unwrap()
    }
}

impl Square<u32> {
    fn new(t: u32) -> Self {
        Square { x: t }
    }
}

impl Square<f64> {
    fn new(t: f64) -> Self {
        Square { x: t }
    }
}

impl Square<String> {
    fn new(t: &str) -> Self {
        Square { x: t.to_owned()}
    }
}

struct Triangle<T>{
    base: T,
    height: T
}

impl Surface<f64> for Triangle<f64>{
    fn area(&self) -> f64{ 
        (self.base*self.height)/2 as f64
    }
}

impl Triangle<f64> {
    fn new(b: f64, h: f64) -> Self {
        Triangle { base: b, height: h }
    }
}

struct Pyramid<T, U>{
    base: T,
    height: U,  
}

impl Volume<f64> for Pyramid<Square<u32>, f64>{
    fn volume(&self) -> f64{ 
        (self.height/3.0)*(self.base.area()) as f64
    }
}

impl Pyramid<Square<u32>, f64> {
    fn new(b: Square<u32>, h: f64) -> Self {
        Pyramid { base: b, height: h }
    }
}


fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());
     
    let triangle = Triangle::new(14.9, 20.1);
    println!("triangle area is {}", triangle.area());
    
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    //let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    //println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
    
}


