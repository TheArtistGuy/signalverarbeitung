use crate::bild_intensitaet::BildIntenstiaet;
use crate::bildbearbeitungs_kernels::gaussche_unschaerfe;

pub fn SIFT (bild : BildIntenstiaet) -> Box<[f32]> {
    //TODO implementieren
    Box::from([0.0])
}

pub fn difference_of_gaussian(bild : BildIntenstiaet) -> BildIntenstiaet{
    let gauss = gaussche_unschaerfe(&bild);
    let mut dog = BildIntenstiaet::new(bild.hoehe, bild.breite);
    for i in 0..bild.hoehe{
        for j in 0..bild.breite{
            dog.set_pixel(i,j,
                bild.get_pixel(i,j).unwrap() - dog.get_pixel(i,j).unwrap());
        }
    }
    dog
}

