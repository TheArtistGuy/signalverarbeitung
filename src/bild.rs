use crate::pixel::RGB;

pub struct Bild8Bit {
    pixel : Vec<RGB>,
    pub hoehe : usize,
    pub breite : usize,
}

impl Bild8Bit {
    ///initialisiert ein neues Bild mit nur weißen Pixeln
    pub fn new(hoehe : usize, breite : usize) -> Self{
        Bild8Bit {
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
        if x > self.hoehe || y > self.breite{
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
    pub fn to_greyscale(&self) -> BildGrauwerte {
        let mut bild = BildGrauwerte::new(self.hoehe, self.breite);
        for x in 0..self.hoehe{
            for y in 0..self.breite{
                bild.set_pixel(x,y, self.get_pixel(x,y).unwrap().to_greyscale())
            }
        }
        bild
    }

    pub fn erstelle_teilbild(&self, x_anfang : usize, x_ende : usize, y_anfang : usize, y_ende : usize) -> Option<Bild8Bit>{
        if x_ende > self.hoehe || y_ende > self.breite{
            return None
        }
        let mut teilbild = Bild8Bit::new(&x_ende - &x_anfang, &y_ende - &y_anfang);
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
}


pub struct BildGrauwerte {
    pixel : Vec<u8>,
    pub hoehe : usize,
    pub breite : usize
}

impl BildGrauwerte {
    pub fn new (hoehe : usize, breite : usize) -> Self{
        BildGrauwerte {
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