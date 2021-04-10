use crate::color_conversion::pixel_rgb_to_greyscale;

pub struct Bild_8Bit {
    pixel : Vec<RGB>,
    pub hoehe : usize,
    pub breite : usize,
}

impl Bild_8Bit{
    ///initialisiert ein neues Bild mit nur weißen Pixeln
    pub fn new(hoehe : usize, breite : usize) -> Self{
        Bild_8Bit{
            pixel: vec![RGB {
                red: 255,
                green: 255,
                blue: 255
            }; hoehe*breite],
            hoehe,
            breite
        }
    }

    ///Gibt den Wert eines Pixels zurück, anders als Mathematiker nutzen wir Höhe x Breite
    pub fn get_pixel(&self, x : usize, y : usize) -> Option<&RGB>{
        if (x > self.hoehe || y > self.breite){
            None
        } else {
            Some(&self.pixel[x + self.breite*x])
        }

    }
    ///Setzt den Wert eines Pixels, anders als Mathematiker nutzen wir Höhe x Breite
    pub fn set_pixel(&mut self, x : usize, y : usize, wert : RGB){
        if !(x > self.hoehe || y > self.breite){
            self.pixel[x + self.breite*x] = wert
        }
    }

    ///Konvertiert das Bild in Grauwerte
    pub fn to_greyscale(&self) -> Bild_Grauwerte{
        let mut bild = Bild_Grauwerte::new(self.hoehe, self.breite);
        for x in 0..self.hoehe{
            for y in 0..self.breite{
                bild.set_pixel(x,y, pixel_rgb_to_greyscale(self.get_pixel(x,y).unwrap()))
            }
        }
        bild
    }

}


pub struct Bild_Grauwerte{
    pixel : Vec<u8>,
    pub hoehe : usize,
    pub breite : usize
}

impl Bild_Grauwerte{
    pub fn new (hoehe : usize, breite : usize) -> Self{
        Bild_Grauwerte{
            pixel: vec![255; hoehe*breite],
            hoehe,
            breite
        }
    }

    ///Gibt den Wert eines Pixels zurück, anders als Mathematiker nutzen wir Höhe x Breite
    pub fn get_pixel(&self, x : usize, y : usize) -> Option<&u8>{
        if x > self.hoehe || y > self.breite {
            None
        } else {
            Some(&self.pixel[x + self.breite*x])
        }

    }
    ///Setzt den Wert eines Pixels, anders als Mathematiker nutzen wir Höhe x Breite
    pub fn set_pixel(&mut self, x : usize, y : usize, wert : u8){
        if !(x > self.hoehe || y > self.breite){
            self.pixel[x + self.breite*x] = wert
        }
    }
}



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

pub struct HSV {
    pub(crate) hue : u32,
    pub(crate) saturation : f32,
    pub(crate) value : f32
}