pub struct Color
{
    pub r : f64,
    pub g : f64,
    pub b : f64
}

impl Color
{
    pub fn new(hexCode : i32) -> Self
    {
        Color
        {
            r : ((hexCode >> 16) & 0xFF) / 255.0,
            b : ((hexCode >> 8) & 0xFF) / 255.0,
            g : (hexCode & 0xFF) / 255.0
        }
    }

}