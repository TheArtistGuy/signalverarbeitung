use crate::bild_intensitaet::BildIntenstiaet;

pub fn gaussche_unschaerfe(bild : &BildIntenstiaet) -> BildIntenstiaet {
    let mut neues_bild = BildIntenstiaet::new(bild.hoehe, bild.breite);
    let faltungskern = Faltungskern3x3::gauss();
    for row in 0..bild.hoehe{
        for col in 0..bild.breite{
            //äussere pixel ignorieren
            if row == 0 || row == bild.hoehe -1 ||col == 0 || col == bild.breite{
                neues_bild.set_pixel(row, col, bild.get_pixel(row, col).unwrap().clone())
            } else{

                let x : f32 = bild.get_pixel(row-1, col-1).unwrap() * faltungskern.matrix[0][0]
                    + bild.get_pixel(row-1, col +1 ).unwrap() * faltungskern.matrix[0][2]
                    + bild.get_pixel(row, col-1).unwrap() * faltungskern.matrix[1][0]
                    + bild.get_pixel(row, col).unwrap() * faltungskern.matrix[1][1]
                    + bild.get_pixel(row, col+1).unwrap() * faltungskern.matrix[1][2]
                    + bild.get_pixel(row+1, col).unwrap() * faltungskern.matrix[2][0]
                    + bild.get_pixel(row+1, col-1).unwrap() * faltungskern.matrix[2][1]
                    + bild.get_pixel(row+1, col+1).unwrap() * faltungskern.matrix[2][2];

                neues_bild.set_pixel(row, col, x);
            }

        }
    }
    neues_bild
}

struct Faltungskern3x3 {
    pub matrix  : [[f32; 3]; 3],
}

impl Faltungskern3x3 {
    fn kantendetektion_prewitt_cols() -> Self{
        Faltungskern3x3 {
            matrix: [[-1.0,-1.0,-1.0],[0.0, 0.0, 0.0], [1.0,1.0,1.0]]
        }
    }
    fn kantendetektion_prewitt_rows() -> Self{
        Faltungskern3x3 {
            matrix: [[-1.0, 0.0, 1.0],[-1.0, 0.0, 1.0],[-1.0, 0.0, 1.0]]
        }
    }
    fn kantendetektion_sobel_cols() -> Self{
        Faltungskern3x3 {
            matrix: [[-1.0,-2.0,-1.0],[0.0, 0.0, 0.0], [1.0,2.0,1.0]]
        }
    }
    fn kantendetektion_sobel_rows() -> Self{
        Faltungskern3x3 {
            matrix: [[-1.0, 0.0, 1.0],[-2.0, 0.0, 2.0],[-1.0, 0.0, 1.0]]
        }
    }
    fn gauss() -> Self{
        Faltungskern3x3{
            matrix: [[(1.0 / 16.0), (2.0 / 16.0), (1.0 / 16.0)], [(2.0 / 16.0), (4.0 / 16.0), (2.0 / 16.0)], [(1.0 / 16.0), (2.0 / 16.0), (1.0 / 16.0)]]
        }
    }
}