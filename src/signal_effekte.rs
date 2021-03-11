pub fn echo (signal : Vec<f32>, verzoegerung : usize, amplitude : f32) ->  Vec<f32>{

    let mut output : Vec<f32> = Vec::new();
    for (index, abtastpunkt) in signal.iter().enumerate(){
        let punkt = (index, abtastpunkt);
        let ausgabepunkt  = if index <= verzoegerung {
            abtastpunkt.clone()
        } else {
                abtastpunkt + amplitude * signal.get(index - verzoegerung).unwrap().clone()
        };
        output.push(ausgabepunkt);
    }
    for abtastpunkt in signal[(signal.len() - verzoegerung) .. ].iter(){
        output.push(*abtastpunkt)
    }
    output
}


