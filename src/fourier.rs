use std::intrinsics::{cosf32, sinf32};
use std::f32::consts::PI;

pub fn berechne_reellwertige_DFT(signal : &[f32]) -> Reellwertige_DFT {
    let mut reX = Vec::new();
    let mut imX = Vec::new();

    for k in 0 .. signal.len() / 2{
        let realteil =  unsafe {
            r = 0.0;
            for (n, wert) in signal.iter().enumerate(){
                r = r  + (wert* cosf32(2* std::f32::consts::PI * k*n / signal.len()));
            }
            r
        };
        let imaginaerteil =  unsafe {
            i = 0.0;
            for (n, wert) in signal.iter().enumerate(){
                i = i + (wert* sinf32(2* PI * k*n / signal.len()));
            }
            i
        };
        reX.push(realteil);
        imX.push(imaginaerteil);
    }

    Reellwertige_DFT{
        re: reX[..],
        im: imX[..]
    }
}



struct Reellwertige_DFT{
    re : [f32],
    im : [f32]
}