/// 3x3 Kernel für gaussches Weichzeichnen

fn gauss_3x3 (input : &[[u8; 3]; 3]) -> f32{
    let mut sum  = 0.0;
    let matrix_3x3 = matrix_3x3 {
        matrix: [[(1.0 / 16.0), (2.0 / 16.0), (1.0 / 16.0)], [(2.0 / 16.0), (4.0 / 16.0), (2.0 / 16.0)], [(1.0 / 16.0), (2.0 / 16.0), (1.0 / 16.0)]]
    };
    for x in 0..2 {
        for y in 0..2{
            sum = sum + f32::from(input[x][y])  * matrix_3x3.matrix[x][y];
        }
    }
    sum

}



struct matrix_3x3 {
    matrix  : [[f32; 3]; 3],
}