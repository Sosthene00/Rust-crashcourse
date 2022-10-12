fn main() {
    let square = Square::<u32>::new(5);
    // let square_float = Square::<f64>::new(5.4);
    // let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    // println!("square_float area is {}", square_float.area());
    // println!("square_string area is {}", square_string.area());

    // let triangle = Triangle::new(14.9, 20.1);
    // let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    // let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    // println!("pyramid_square volume is {}", pyramid_square.volume());
    // println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}

pub trait Form {
    pub fn area();
    pub fn volume();
}

struct Square{
    edge_length: u32;
}

impl Form for Square {
    pub fn area(&self){
        let result = *self*self;
        return result;
    }
}