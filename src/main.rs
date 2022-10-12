
//////////// Trait /////////////////////////////////
trait Area {
    fn area(&self) -> f64;
}

trait Volume {
    fn volume(&self) -> f64;
}

///////////////////// Struct //////////////////////////////
struct Square<T> { size: T}
struct Triangle<T> { base : T, height: T}
struct Pyramid<T, U> { object : T, length : U}

/////////////////////impl Square ///////////////////////////
impl Square<f64> {
    fn new( value: f64) -> Self {
        Square { size : value}
    }
}

impl Square<u32> {
    fn new( value: u32) -> Self {
        Square { size : value}
    }

}

impl Square<String> {
    fn new( value: &str) -> Self {
        Square { size : value.to_string()}
    }
}

/////////////////////impl Triangle ///////////////////////////
impl Triangle<f64> 
{
    fn new(value1:f64,value2:f64) -> Self {
        Triangle { base : value1, height : value2}
    }
}

/////////////////////impl Pyramid ///////////////////////////
impl Pyramid<Square<u32>, f64> {
    fn new(object : Square<u32>, length : f64) -> Self {
        Pyramid { object : object, length : length}
    }
}

impl Pyramid<Triangle<f64>, f64> {
    fn new(object : Triangle<f64>, length : f64) -> Self {
        Pyramid { object : object, length : length}
    }
}


///////////////////////// impl Area ///////////////////////////////////
impl Area for Square<f64> {
    fn area(&self) -> f64 {
        self.size * self.size
    }
}

impl Area for Square<u32> {
    fn area(&self) -> f64 {
        self.size as f64 * self.size as f64
    }
}


impl Area for Square<String> {
    fn area(&self) -> f64 {
        let value_float : f64 = self.size.parse::<f64>().unwrap();
        value_float*value_float
    }
}

impl Area for Triangle<f64> {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

////////////////////////   impl Volume ////////////////////////////

impl Volume for Pyramid<Square<u32>, f64> {
    fn volume(&self) -> f64 {
        self.object.area() * self.length / 3.0
    }
}

impl Volume for Pyramid<Triangle<f64>, f64> {
    fn volume(&self) -> f64 {
        self.object.area() * self.length / 3.0
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
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
