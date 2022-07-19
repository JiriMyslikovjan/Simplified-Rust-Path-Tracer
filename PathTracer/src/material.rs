use crate::color::Color;

// Enum for different surface types, important for BRDF in the rendering equation
pub enum MatType
{
    diffuse,
    specular,
    refractive
}

pub struct Material
{
    pub color : Color,
    pub mType : MatType,
    pub emission : f64
}

impl Material
{
    pub fn new(clr : i32, matType : MatType, matEmission : f64) -> Self
    {
        Material
        {
            color : Color::new(0x0000FF),
            mType : matType,
            emission : matEmission
        }
    }
}