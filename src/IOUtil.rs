use std::fs::File;
use std::io;
use std::io::{self, prelude::*, BufReader};
use std::fmt::Display;



trait DictLoader {
     fn load_from(path:String) -> Vec<String>;
}

struct FileDictLoader {

}

impl DictLoader for FileDictLoader {
      fn load_from(path:String) -> Vec<String> {
    let file= File::open(path);
    let reader = BufReader::new(file);
    let mut dict_strings: Vec <String> = Vec::new();
    for line in reader.lines().unwrap(){
        dict_strings.push(line);
    }

    Ok(());
   return dict_strings; 
}
}


