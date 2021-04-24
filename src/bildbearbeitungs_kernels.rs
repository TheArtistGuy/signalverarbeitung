



struct Faltungskern3x3 {
    matrix  : [[f32; 3]; 3],
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