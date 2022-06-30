#[derive(Debug)]
pub struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn zero() -> Vector {
        return Vector { x: 0, y: 0 };
    }

    pub fn from(x: i32, y: i32) -> Vector {
        return Vector { x, y };
    }

    pub fn add(a: Vector, b: Vector) -> Vector {
        return Vector {
            x: a.x + b.x,
            y: a.y + b.y,
        };
    }

    pub fn scale(self, factor: i32) -> Vector {
        return Vector {
            x: self.x * factor,
            y: self.y * factor,
        };
    }

    pub fn add_x(&mut self, num: i32) {
        self.x += num;
    }

    pub fn add_y(&mut self, num: i32) {
        self.y += num;
    }
}

pub fn run() {
    let left = Vector::zero();
    let right = Vector::from(3, 1);
    let mut sum = Vector::add(left, right).scale(5);

    sum.add_x(10);
    sum.add_y(10);

    println!("{:?}", sum);
}
