use std::ops::Mul;

pub trait Surface {
    fn area(&self) -> u32;

    //fn area_float(&self) -> f64;
}

struct Square<T>{
    x: T
}


impl<T> Surface for Square<T>{
    fn area(&self) -> u32{
        self.x
    }

    /* 
    fn area_float(&self) -> f64{
        self.x.pow(2)
    }
    */
}

/*
impl Surface for Square<u32>{
    fn area(&self) -> f64{
        self.x
    }
}

impl Surface for Square<f64>{
    fn area_float(&self) -> f64{
        self.x*self.x
    }
}
*/

struct Triangle<T>{
    x: T
}

struct Pyramid<T>{
    x: T
}

fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());
    /* 
    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
    */
}


