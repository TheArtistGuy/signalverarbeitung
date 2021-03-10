
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
}