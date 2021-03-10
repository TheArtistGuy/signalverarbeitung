

pub fn pixel_rgb_to_greyscale(rgb : &RGB) -> u8 {
    (rgb.red as f32 * 0.3 + rgb.green as f32 * 0.59 + rgb.blue as f32 * 0.11) as u8
}


pub fn pixel_rgb_to_hsv (rgb : &RGB) -> HSV {
    let red_percentage : f32 = (rgb.red / 255) as f32;
    let green_percentage:f32 = (rgb.green / 255) as f32;
    let blue_percentage :f32 = (rgb.blue / 255) as f32;

    let big_m = get_greatest_rgb([&red_percentage, &green_percentage, &blue_percentage]);
    let small_m = get_smallest_rgb([&red_percentage, &green_percentage, &blue_percentage]);

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

fn get_smallest_rgb(elements: [&f32; 3]) -> &f32{
    let mut min = elements.get(0).unwrap();
    for e in elements.iter() {
        if e < min {
            min = e;
        }
    }
    min
}


pub fn pixel_hsv_to_rgb(hsv : &HSV) -> Result<RGB, &str> {

    let h : u32 = hsv.hue / 60;

    let f : f32 = (hsv.hue / 60) as f32 - h as f32;

    let p : u8 = ((hsv.value * (1.0 - hsv.saturation)) * 255.0) as u8;
    let q : u8 = ((hsv.value * (1.0 - hsv.saturation * f)) * 255.0) as u8;
    let t : u8 = ((hsv.value * (1.0 - hsv.saturation * (1.0-f))) * 255.0) as u8;
    let v : u8 = (hsv.value * 255.0) as u8;

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


fn get_greatest_rgb(elements : [&f32; 3]) -> &f32 {
    let mut max = elements.get(0).unwrap();
    for e in elements.iter() {
        if e > max {
            max = e;
        }
    }
    max
}

pub struct RGB {
    red : u8,
    green : u8,
    blue : u8
}

pub struct HSV {
    hue : u32,
    saturation : f32,
    value : f32
}



#[cfg(test)]
mod tests {
    use crate::color_conversion::{get_greatest_rgb, get_smallest_rgb, RGB, pixel_hsv_to_rgb, HSV, pixel_rgb_to_greyscale};

    #[test]
    fn greater_test() {
        assert_eq!(get_greatest_rgb([&1.0 , &4.2, &4.0]), &(4.2 as f32));
    }
    #[test]
    fn smaller_test() { assert_eq!(get_smallest_rgb([&1.0, &4.2, &4.0]), &(1.0 as f32));}

    #[test]
    fn check_rounding() {assert_eq!(12.7 as u8, 12);}

    #[test]
    fn test_greyscale(){assert_eq!(pixel_rgb_to_greyscale(&RGB {
        red: 255,
        green: 255,
        blue: 255
    }), 255)}
}
