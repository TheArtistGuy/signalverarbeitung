use crate::bild::{Bild8Bit};

pub struct BildIntenstiaet {
    pixel : Vec<f32>,
    pub hoehe : usize,
    pub breite : usize,
}

impl BildIntenstiaet {

    pub fn new (hoehe : usize, breite : usize) -> Self{
        Self{
            pixel: vec![0.0; hoehe * breite],
            hoehe,
            breite
        }
    }

    pub fn from_helligkeit(bild : Bild8Bit) -> Self{
        let greyscale = bild.to_greyscale();
        let mut zellen = Vec::new();
        for i in 0..greyscale.hoehe{
            for j in 0..greyscale.breite{
                let x = *greyscale.get_pixel(i,j).unwrap() as f32 /255.0;
                zellen.push(x);
            }
        }
        Self{
            pixel: zellen,
            hoehe: bild.hoehe,
            breite: bild.breite
        }
    }

    pub fn from_saettigung(bild : Bild8Bit) -> Self{
        let mut zellen = Vec::new();
        for i in 0..bild.hoehe{
            for j in 0..bild.breite{
                let x = bild.get_pixel(i,j).unwrap().to_hsv().saturation;
                zellen.push(x);
            }
        }
        Self{
            pixel: zellen,
            hoehe: bild.hoehe,
            breite: bild.breite
        }
    }
    ///Gibt den Wert eines Pixels zurück, anders als Mathematiker nutzen wir Höhe x Breite
    pub fn get_pixel(&self, x : usize, y : usize) -> Option<&f32>{
        if x > self.hoehe || y > self.breite{
            None
        } else {
            Some(&self.pixel[x + self.breite*x])
        }

    }
    ///Setzt den Wert eines Pixels, anders als Mathematiker nutzen wir Höhe x Breite
    pub fn set_pixel(&mut self, x : usize, y : usize, wert : f32){
        if !(x > self.hoehe || y > self.breite){
            self.pixel[x + self.breite*x] = wert
        }
    }

    pub fn get_element(&self, elem : usize) -> Option<&f32> {
        if  elem > self.breite * self.hoehe || elem < 0{
            return None
        }
        Some(&self.pixel[elem])
    }

    pub fn set_element(&mut self, elem: usize, wert : f32) {
        if  elem > self.breite * self.hoehe || elem < 0 {
            panic!()
        }
        self.pixel[elem] = wert.clone();
    }

    pub fn erstelle_teilbild(&self, x_anfang : usize, x_ende : usize, y_anfang : usize, y_ende : usize) -> Option<BildIntenstiaet>{
        if x_ende > self.hoehe || y_ende > self.breite{
            return None
        }
        let mut teilbild = BildIntenstiaet::new(&x_ende - &x_anfang, &y_ende - &y_anfang);
        for x in x_anfang..x_ende{
            for y in y_anfang..y_ende{
                let x_neues_bild = x-x_anfang;
                let y_neues_bild = y-y_anfang;

                teilbild.set_pixel(x_neues_bild, y_neues_bild, match self.get_pixel(x,y) {
                    Some(c) => c.clone(),
                    _ => panic!()
                });
            }
        }
        Some(teilbild)
    }
    pub fn erstelle_integral_image(&self, x_anfang : usize, x_ende : usize, y_anfang : usize, y_ende : usize) -> Option<BildIntenstiaet> {
        let teilbild = self.erstelle_teilbild(x_anfang, x_ende, y_anfang, y_ende);
        match teilbild {
            Some(f) => {
                let mut integral_image = BildIntenstiaet::new(f.hoehe, f.breite);
                integral_image.set_element(0, (f.get_element(0).unwrap()).clone());

                for y in 1..integral_image.breite {
                    let mut sum_of_row = 0.0;
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

}