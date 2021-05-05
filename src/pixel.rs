pub struct RGB {
    pub red : u8,
    pub green : u8,
    pub blue : u8
}

impl Clone for RGB{
    fn clone(&self) -> Self {
        RGB{
            red: self.red.clone(),
            green: self.green.clone(),
            blue: self.blue.clone()
        }
    }
}

impl RGB {
    pub fn to_greyscale(&self) -> u8 {
        (self.red as f32 * 0.3 + self.green as f32 * 0.59 + self.blue as f32 * 0.11) as u8
    }

    pub fn to_hsv (&self) -> HSV {
        let red_percentage : f32 = (self.red / 255) as f32;
        let green_percentage:f32 = (self.green / 255) as f32;
        let blue_percentage :f32 = (self.blue / 255) as f32;

        let big_m = RGB::get_greatest([&red_percentage, &green_percentage, &blue_percentage]);
        let small_m = RGB::get_smallest([&red_percentage, &green_percentage, &blue_percentage]);

        let hue = if big_m == small_m {
            0
        } else if  *big_m == red_percentage{
            (60.0 * (0.0 + ((green_percentage - blue_percentage) / (big_m - small_m))))  as u32 % 360
        } else if *big_m == green_percentage{
            (60.0 * (2.0 + ((blue_percentage - red_percentage) / (big_m - small_m)))) as u32
        } else {
            (60.0 * (4.0 + ((red_percentage - green_percentage) / (big_m - small_m))) )as u32
        };

        let saturation = if *big_m == 0.0 {
            0.0
        } else{
            (big_m - small_m) / big_m
        };

        let value = big_m.clone();

        HSV{
            hue,
            saturation,
            value
        }
    }

    fn get_smallest(elements: [&f32; 3]) -> &f32{
        let mut min = elements.get(0).unwrap();
        for e in elements.iter() {
            if e < min {
                min = e;
            }
        }
        min
    }
    fn get_greatest(elements : [&f32; 3]) -> &f32 {
        let mut max = elements.get(0).unwrap();
        for e in elements.iter() {
            if e > max {
                max = e;
            }
        }
        max
    }
}


pub struct HSV {
    pub(crate) hue : u32,
    pub(crate) saturation : f32,
    pub(crate) value : f32
}

impl HSV {
    pub fn to_rgb(&self) -> Result<RGB, &str> {

        let h : u32 = self.hue / 60;

        let f : f32 = (self.hue / 60) as f32 - h as f32;

        let p : u8 = ((self.value * (1.0 - self.saturation)) * 255.0) as u8;
        let q : u8 = ((self.value * (1.0 - self.saturation * f)) * 255.0) as u8;
        let t : u8 = ((self.value * (1.0 - self.saturation * (1.0-f))) * 255.0) as u8;
        let v : u8 = (self.value * 255.0) as u8;

        let rgb = match h {
            0 => Ok(RGB{
                red: v,
                green: t,
                blue: p
            }),
            1 => Ok(RGB{
                red: q,
                green: v,
                blue: p
            }),
            2 => Ok(RGB {
                red: p,
                green: v,
                blue: t
            }),
            3 => Ok(RGB{
                red: p,
                green: q,
                blue: v
            }),
            4 => Ok(RGB{
                red: t,
                green: p,
                blue: v
            }),
            5 => Ok(RGB{
                red: v,
                green: p,
                blue: q
            }),
            _ => Err("konnte nicht umgewandelt werden"),
        };
        rgb
    }
}




#[cfg(test)]
mod tests {
    use crate::pixel::RGB;

    #[test]
    fn greater_test() {
        assert_eq!(RGB::get_greatest([&1.0, &4.2, &4.0] ), &(4.2 as f32))
    }
    #[test]
    fn smaller_test() {
        assert_eq!(RGB::get_smallest([&1.0, &4.2, &4.0]), &(1.0 as f32));}

    #[test]
    fn check_rounding() {assert_eq!(12.7 as u8, 12);}

    #[test]
    fn test_greyscale(){assert_eq!(RGB {
        red: 255,
        green: 255,
        blue: 255
    }.to_greyscale(), 255)}
}
