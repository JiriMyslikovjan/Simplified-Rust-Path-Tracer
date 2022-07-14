pub struct Color
{
    pub r : f64,
    pub g : f64,
    pub b : f64
}

impl Color
{
    // Converts hexadecimal values to RGB values
    // TODO: Color clamping, gamma correction
    pub fn new(hexCode : i32) -> Self
    {
        Color
        {
            r : ((hexCode >> 16) & 0xFF) as f64 / 255.0,
            g : ((hexCode >> 8) & 0xFF) as f64 / 255.0,
            b : (hexCode & 0xFF) as f64 / 255.0
        }
    }

}