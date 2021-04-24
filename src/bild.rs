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

    pub fn get_element(&self, elem : usize) -> Option<&RGB> {
        if  elem > self.breite * self.hoehe || elem < 0{
            return None
        }
        Some(&self.pixel[elem])
    }

    pub fn set_element(&mut self, elem: usize, wert : &RGB) {
        if  elem > self.breite * self.hoehe || elem < 0 {
            panic!()
        }
        self.pixel[elem] = wert.clone();
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

    pub fn erstelle_teilbild(&self, x_anfang : usize, x_ende : usize, y_anfang : usize, y_ende : usize) -> Option<Bild_8Bit>{
        if x_ende > self.hoehe || y_ende > self.breite{
            return None
        }
        let mut teilbild = Bild_8Bit::new(&x_ende - &x_anfang, &y_ende - &y_anfang);
        for x in x_anfang..x_ende{
            for y in y_anfang..y_ende{
                let x_neues_bild = x-x_anfang;
                let y_neues_bild = y-y_anfang;

                teilbild.set_pixel(x_neues_bild, y_neues_bild, match self.get_pixel(x,y) {
                    Some(c) => c.clone(),
                    _ => panic!()
                        /*RGB{
                        red: 0,
                        green: 0,
                        blue: 0
                    }*/
                });
            }
        }
        Some(teilbild)
    }
    /*
    TODO : implementation für intensitätswerte


    pub fn erstelle_integral_image(&self, x_anfang : usize, x_ende : usize, y_anfang : usize, y_ende : usize) -> Option<Bild_8Bit> {
        let teilbild = self.erstelle_teilbild(x_anfang, x_ende, y_anfang, y_ende);
        match teilbild {
            Some(f) => {
                let mut integral_image = Bild_8Bit::new(f.hoehe, f.breite);
                integral_image.set_element(0, (f.get_element(0).unwrap()).clone);

                for y in 1..integral_image.breite {
                    let mut sum_of_row = 0;
                    for x in 1..integral_image.hoehe {
                        sum_of_row = sum_of_row + f.get_element(x*f.hoehe+y).unwrap();
                        integral_image.set_element(x*f.hoehe+y, integral_image.get_element(x-1*f.breite + y).unwrap() + sum_of_row);
                    }
                }

                Some(integral_image)
                },
            _ => None
        }
    }
    */
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