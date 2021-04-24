use std::f32::consts::PI;

pub fn berechne_reellwertige_DFT(signal : &[f32]) -> ReellwertigeDFT {
    let mut re_x = Vec::new();
    let mut im_x = Vec::new();

    for k in 0 .. signal.len() / 2{
        let realteil =   {
            let mut r = 0.0;
            for (n, wert) in signal.iter().enumerate(){
                r = r  + (wert* f32::cos(2.0* PI * (k*n) as f32 / signal.len() as f32));
            }
            r
        };
        let imaginaerteil =   {
            let mut i = 0.0;
            for (n, wert) in signal.iter().enumerate(){
                i = i + (wert* f32::sin(2.0* PI * (k*n) as f32 / signal.len() as f32));
            }
            i
        };
        re_x.push(realteil);
        im_x.push(imaginaerteil);
    }

    ReellwertigeDFT {
        re: re_x,
        im: im_x
    }
}



struct ReellwertigeDFT {
    re : Vec<f32>,
    im : Vec<f32>
}