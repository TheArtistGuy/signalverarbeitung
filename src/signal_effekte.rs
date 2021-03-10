pub fn echo (signal : Vec<f32>, verzoegerung : usize, amplitude : f32) ->  Vec<f32>{

    let mut output = Vec::new();
    for (index, abtastpunkt) in signal.iter().enumerate(){
        let punkt = (index, abtastpunkt);
        let ausgabepunkt = match punkt {
            ( 0..verzoegerung, x) => x.clone(),
            (i, x)  =>
                x+ amplitude * signal.get(i - verzoegerung).unwrap(),
        };
        output.push(ausgabepunkt);
    }
    for abtastpunkt in signal[(signal.len() - verzoegerung) .. ].iter(){
        output.push(*abtastpunkt)
    }
    output
}


