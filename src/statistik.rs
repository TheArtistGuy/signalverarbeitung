
fn arithmetisches_mittel(signal : &[f32]) -> f32{
    let mut summe : f32 = 0.0;
    for messpunkt in signal{
        summe = summe + messpunkt;
    }
    summe / signal.len() as f32
}

fn mittlere_absolute_abweichung (signal : &[f32]) -> f32{
    let x_strich = arithmetisches_mittel(&signal);
    let mut summe: f32 = 0.0;
    for messpunkt in signal{
        summe = summe + (messpunkt - &x_strich).abs();
    }
    summe / signal.len() as f32
}

fn varianz (signal : &[f32]) -> f32{
    let x_strich = arithmetisches_mittel(&signal);
    let mut summe: f32 = 0.0;
    for messpunkt in signal{
        summe = summe + (messpunkt - &x_strich).powi(2);
    }
    summe / signal.len() as f32
}

fn varianz_geschaetzt(signal: &[f32]) -> f32{
    let x_strich = arithmetisches_mittel(&signal);
    let mut summe: f32 = 0.0;
    for messpunkt in signal{
        summe = summe + (messpunkt - &x_strich).powi(2);
    }
    summe / (signal.len() -1) as f32
}
fn standardabweichung (signal :&[f32]) -> f32{
    varianz(signal).sqrt()
}

fn standardabweichung_geschaetzt(signal :&[f32]) -> f32{
    varianz_geschaetzt(signal).sqrt()
}

fn median (signal : Box<[f32]>) -> f32{
    let mut signal_copy = signal.clone();
    signal_copy.sort_by(|x1,x2| x1.partial_cmp(x2).unwrap());
    let median = match signal_copy.len() %2 {
      0 => (signal_copy.get(signal.len()/2).unwrap()  + signal_copy.get((signal.len()/2) - 1).unwrap()) /2.0, //kein +1, da von groß nach klein geordnet
      _ => *signal_copy.get(signal_copy.len()/2).unwrap()
    };
    median.clone()
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetisches_mittel() {
        let signal: [f32; 3] = [3.0, 1.0, 5.0];
        assert_eq!(arithmetisches_mittel(&signal), 3.0);
    }

    #[test]
    fn test_maa() {
        let signal: [f32; 3] = [3.0, 1.0, 5.0];
        assert_eq!(mittlere_absolute_abweichung(&signal), 4.0 / 3.0);
    }

    #[test]
    fn test_varianz() {
        let signal: [f32; 3] = [3.0, 1.0, 5.0];
        assert_eq!(varianz(&signal), 8.0 / 3.0);
    }

    #[test]
    fn test_median_1() {
        let signal:[f32;5] = [3.2, 5.5,7.2,10.0,4.0];
        assert_eq!(median(Box::new(signal)), 5.5 as f32);
    }

    #[test]
    fn test_median_2(){
        let signal :[f32;6] = [3.2, 5.5,7.2,10.0,4.0, 12.1];
        assert_eq!(median(Box::new(signal)), 6.35 as f32);
        let signal  = [1.0,1.0,2.0,2.0];
        assert_eq!(median(Box::new(signal)), 1.5 as f32)
    }

}