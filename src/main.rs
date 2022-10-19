mod shape;
use crate::shape::{MesurableArea, MesurableVolume, Shape2D, Shape3D};

fn main() {
    let square = Shape2D::Square(5.0);
    let triangle =Shape2D::Triangle(5.0,4.0);
    let circle =Shape2D::Circle(6.0);

    println!("square area is {}", square.area());
    println!("triangle area is {}", triangle.area());

    let pyramid_square = Shape3D::Pyramid(square,7.0);
    let pyramid_triangle= Shape3D::Pyramid(triangle,6.0);
    let cone=Shape3D::Pyramid(circle,7.0);
    let sphere=Shape3D::Sphere(5.0);

    let pyramidfrom2d=triangle.get_pyramid(6.0);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
    println!("cone volume is {}", cone.volume());
    println!("sphere volume is {}", sphere.volume());
    println!("pyramid from 2D volume is {}", pyramidfrom2d.volume());


    /*let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<&str>::new("6");


    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());*/
}

