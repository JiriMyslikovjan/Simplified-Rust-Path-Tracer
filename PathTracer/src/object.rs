use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait  Object
{
    fn intersect(&self, ray : Ray) -> f64;
    fn normal(&self, vec : Vector) -> Vector;
}

pub struct Sphere
{
    pub centre : Vector,
    pub radius : f64,
    pub material : Material,
}

impl Object for Sphere
{
    fn intersect(&self, ray: Ray) -> f64
    {
        let sCentre = self.centre;
        let sRadius = self.radius;
        let rOrigin = ray.origin;
        let rDest = ray.dest;

        let b = ((rOrigin - sCentre) * 2.0).dotProduct(rDest);
        let c = (rOrigin - sCentre).dotProduct(rOrigin - sCentre) - (&sRadius * &sRadius);

        let discr = b * b - 4.0 * c;

        if discr < 0.0
        {
            return 0.0;
        }
        let result1 = -b + discr;
        let result2 = -b - discr;

        if result2 > 1e-6
        {
            return result2 / 2.0;
        }
        else if  result1 > 1e-6
        {
            return result1 / 2.0;
        }

        return 0.0
    }

    fn normal(&self, vec : Vector) -> Vector
    {
        let sCentre = self.centre;

        return *(vec - sCentre).normalize();
    }
}
