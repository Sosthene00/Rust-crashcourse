#[derive(Debug, Copy, Clone)]
pub enum Shape2D {
    Square(f64),
    Triangle(f64, f64),
    Circle(f64)
}

pub trait MesurableArea {
    fn area(&self) -> f64;
    fn get_pyramid(&self,height: f64) -> Shape3D;
}

impl MesurableArea for Shape2D {
    fn area(&self) -> f64 {
        match &self {
            Shape2D::Square(x) => { x * x }
            Shape2D::Triangle(x, y) => { (x * y) / 2.0 }
            Shape2D::Circle(radius) => {radius.powf(2.0)*std::f64::consts::PI}
        }
    }

    fn get_pyramid(&self, height:f64) -> Shape3D{
        Shape3D::Pyramid(*self, height)
    }
}

pub enum Shape3D {
    Pyramid(Shape2D, f64),
    Sphere(f64)
}

pub trait MesurableVolume {
    fn volume(&self) -> f64;
}

impl MesurableVolume for Shape3D {
    fn volume(&self) -> f64 {
        match &self {
            Shape3D::Pyramid(shape2d, h) => { (shape2d.area() * h) / 3.0 }
            Shape3D::Sphere(radius)=>{(4.0*std::f64::consts::PI*radius.powf(3.0))/3.0}
        }
    }
}









