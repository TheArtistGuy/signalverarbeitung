


pub struct Vektor3 {
    a : f32,
    b : f32,
    c : f32
}

impl Vektor3 {

}


pub fn skalarprodukt (x : &Vektor3, y : &Vektor3) -> f32{
    x.a*y.a + x.b*y.b + x.c * y.c
}



pub fn vektorprodukt(x: &Vektor3, y: &Vektor3) -> Vektor3 {
    let a = x.b * y.c - x.c * y.b;
    let b = x.c * y.a - x.a * y.c;
    let c = x.a * y.b - x.b * y.a;

    Vektor3 {a,b,c}
}


#[cfg(test)]
mod tests {
    use crate::vektor_operationen::{skalarprodukt, Vektor3, vektorprodukt};

    #[test]
    fn test_skalarprodukt(){
        let x  = Vektor3 { a: 1.0, b: 0.0,  c: 2.0 };
        let y = Vektor3 { a: 1.0, b: 2.0, c: 1.0 };
        assert_eq!(skalarprodukt(&x, &y), 3.0);
    }

    #[test]
    fn test_vektorprodukt(){
        let x = Vektor3 {a: -1.0, b: 2.0, c : 0.0};
        let y = Vektor3 {a : 3.0, b : -4.0, c : 2.0};
        let ergebnis = vektorprodukt(&x,&y);
        assert_eq!(ergebnis.a, 4.0);
        assert_eq!(ergebnis.b, 2.0);
        assert_eq!(ergebnis.c, -2.0);
    }
}

