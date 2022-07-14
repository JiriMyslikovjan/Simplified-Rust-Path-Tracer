use crate::vector::Vector;

// Structure representing a ray
pub struct Ray
{
    pub origin : Vector,
    pub dest : Vector
}

impl Ray
{
    pub fn new() -> Self
    {
        Ray
        {
            origin : Vector::new(),
            dest : Vector::new()
        }
    }

    pub fn setOrigin(&mut self ,x : f64, y : f64, z : f64) -> &mut Self
    {
        self.origin.setVector(x, y, z);

        self
    }

    pub fn setDest(&mut self ,x : f64, y : f64, z : f64) -> &mut Self
    {
        self.dest.setVector(x, y, z);

        self
    }
}
