
fn rosenbrock_durch_gauss_newton (punkt_1 : f64, punkt_2 : f64, schwellwert : f64) -> (f64, f64){
    let d_1 = 1.0 - punkt_1;
    let d_2 = 2.0*punkt_1 - f64::powi(punkt_1, 2 ) - punkt_2;

    if f64::abs(punkt_1 - d_1) > schwellwert || f64::abs(punkt_2 - d_2) > schwellwert {
        rosenbrock_durch_gauss_newton(d_1,d_2,schwellwert)
    } else {
        (d_1, d_2)
    }
}