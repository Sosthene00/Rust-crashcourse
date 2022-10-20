fn main() {
    pub trait Surface<T> {
        fn area(&self) -> T;
    }

    pub trait Volume<U> {
        fn volume(&self) -> U;
    }

    struct Square<T> {
        x: T,
    }

    struct Triangle<L, H> {
        x: L,
        y: H,
    }

    struct Pyramid<B, H> {
        base: B,
        height: H,
    }

    impl Surface<f64> for Square<u32> {
        fn area(&self) -> f64 {
            (self.x * self.x) as f64
        }
    }

    impl Surface<f64> for Square<f64> {
        fn area(&self) -> f64 {
            self.x * self.x
        }
    }

    impl Surface<f64> for Square<String> {
        fn area(&self) -> f64 {
            self.x.parse::<f64>().unwrap() * self.x.parse::<f64>().unwrap()
        }
    }

    impl Surface<f64> for Triangle<f64, f64> {
        fn area(&self) -> f64 {
            self.x * self.y / 2.0
        }
    }

    impl Volume<f64> for Pyramid<Square<u32>, f64> {
        fn volume(&self) -> f64 {
            (self.base.area() / 3.0) * self.height
        }
    }

    impl Volume<f64> for Pyramid<Triangle<f64, f64>, f64> {
        fn volume(&self) -> f64 {
            (self.base.area() / 3.0) * self.height
        }
    }

    impl Square {
        fn new<T>(l:T) -> Self
        where
            T: Try_into<f64>,
        {
            Self {length: l.try_into().unwrap_or(0,0)}
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
            Square { x: t.to_owned() }
        }
    }

    impl Triangle {
        fn new<T>(a: T, b:T, c:T) -> Self
        where
            T: TryInto<f64>,
        {
            Self {
                a: a.try_into().unwrap_or(0,0),
                b: b.try_into().unwrap_or(0,0),
                c: c.try_into().unwrap_or(0,0),
            }
        }
    }

    impl Triangle<f64, f64> {
        fn new(x: f64, y: f64) -> Self {
            Triangle { x: x, y: y }
        }
    }

    impl Pyramid<Square<u32>, f64> {
        fn new(x: Square<u32>, y: f64) -> Self {
            Pyramid { base: x, height: y }
        }
    }

    impl Pyramid<Triangle<f64, f64>, f64> {
        fn new(x: Triangle<f64, f64>, y: f64) -> Self {
            Pyramid { base: x, height: y }
        }
    }

    let square = Square::<u32>::new(5);
    println!("square area is {}", square.area());

    let square_float = Square::<f64>::new(5.4);
    println!("square_float area is {}", square_float.area());

    let square_string: Square<String> = Square::<String>::new("6");
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    println!("triangle area is {}", triangle.area());

    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    println!("pyramid_square volume is {}", pyramid_square.volume());

    let pyramid_triangle = Pyramid::<Triangle<f64, f64>, f64>::new(triangle, 24.3);
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
