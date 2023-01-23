use std::fs::File;
//use std::io;
use std::io::{self, prelude::*, BufReader};
use std::fmt::Display;
use std::collections::HashMap;

fn main() {
    match read_file("parole.txt".to_string()) {
        Ok(()) => {},
        Err(e) => {println!("Errore {:?}",e);}
    }
}

fn read_file(file:String) -> io::Result<()>{
    let file= File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        println!("{}", line?);
    }

    Ok(())
    
}

fn HashMap_shit(){
    let mut words= HashMap::new();
}



fn fix_string(name: impl Display){
    println!("Prova {}", name);
}
