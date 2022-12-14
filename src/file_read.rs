
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
pub fn read_file_line_by_line(filepath: &str , vec: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let  file = match File::open(filepath) {
        Ok( f) => f,
        Err(err) => panic!("unable to read from file error:{}", err.to_string()),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let x = line.unwrap().to_string();
        vec.push(x);
    }
    Ok(())
}