use std::f64::consts;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    pub fn area(&self) -> f64 {
        consts::PI * (self.radius * self.radius)
    }

    pub fn grow(&self, mult: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: (self.radius * mult) }
    }
}

pub struct CircleBuilder {
    pub coordinate_x: f64,
    pub coordinate_y: f64,
    pub radius: f64,
}

impl CircleBuilder {
    pub fn new() -> CircleBuilder {
        CircleBuilder { coordinate_x: 0.0, coordinate_y: 0.0, radius: 1.0 }
    }

    pub fn coordinate(&mut self, coordinate_x: f64, coordinate_y: f64) -> &mut CircleBuilder {
        self.coordinate_x = coordinate_x;
        self.coordinate_y = coordinate_y;
        self
    }

    pub fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    pub fn finalize(&self) -> Circle {
        Circle { x: self.coordinate_x, y: self.coordinate_y, radius: self.radius }
    }
}

pub struct Square {
    pub x: f64,
    pub y: f64,
    pub side: f64,
}

impl Square {

    pub fn new(x: f64, y: f64, side: f64) -> Square {
        Square { x: x,
                 y: y,
                 side: side,
        }
    }
    
    pub fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub struct SquareBuilder {
    pub x: f64,
    pub y: f64,
    pub side: f64,
}

impl SquareBuilder {
    pub fn new() -> SquareBuilder {
        SquareBuilder { x: 0.0, y: 0.0, side: 1.0 }
    }
    
    pub fn coordinate(&mut self, coordinate_x: f64, coordinate_y: f64) -> &mut SquareBuilder {
        self.x = coordinate_x;
        self.y = coordinate_y;
        self
    }
    
    pub fn side(&mut self, side: f64) -> &mut SquareBuilder {
        self.side = side;
        self
    }
    
    pub fn finalize(&self) -> Square {
        Square{ x: self.x, y: self.y, side: self.side }
    }
}


