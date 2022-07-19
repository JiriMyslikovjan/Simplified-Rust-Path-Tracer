
#[derive(Clone, Copy)]
pub struct Vector
{
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl Vector
{
    pub fn new() -> Self
    {
        Vector
        {
            x : 0.0,
            y : 0.0,
            z : 0.0
        }
    }

    pub fn setVector(&mut self, x : f64, y : f64, z : f64) -> &mut Self
    {
        self.x = x;
        self.y = y;
        self.z = z;

        self
    }

    pub fn mulByVec(&self, vec2 : Vector) -> Vector
    {
        Vector
        {
            x : self.x * vec2.x,
            y : self.y * vec2.y,
            z : self.z * vec2.z
        }
    }

    pub fn normalize(&mut self) -> &mut Self
    {
        let vecLen : f64 = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        self.x = self.x / vecLen;
        self.y = self.y / vecLen;
        self.z = self.z / vecLen;

        self
    }

    pub fn dotProduct(&self, vec2 : Vector) -> f64
    {
        self.x * vec2.x + self.y * vec2.y + self.z * vec2.z
    }
}

impl std::ops::Add for Vector{
    type Output = Vector;

    fn add(self, vec2: Vector) -> Vector {
        Vector
        {
            x : self.x + vec2.x,
            y : self.y + vec2.y,
            z : self.z + vec2.z
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, vec2: Vector) -> Vector {
        Vector
        {
            x : self.x - vec2.x,
            y : self.y - vec2.y,
            z : self.z - vec2.z
        }
    }
}

impl  std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, constant : f64) -> Vector {
        Vector
        {
            x : self.x * constant,
            y : self.y * constant,
            z : self.z * constant
        }
    }
}