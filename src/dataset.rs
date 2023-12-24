
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
pub struct DataSet {
    pub input: Vec<f64>,
    pub output: Vec<f64>,
    pub size: usize,
}

#[allow(dead_code)]
impl DataSet {
    pub fn new(path: &Path) -> DataSet {
        
        if let Ok(lines) = read_lines(path) {
            
            let mut input_vec: Vec<f64> = Vec::new();
            let mut output_vec: Vec<f64> = Vec::new();

            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    // println!("{:?}", ip);
    
                    let line: Vec<&str> = ip.split(",").collect();
    
                    let output: f64 = match line[0].parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
    
                    // set input and validate if it is a number, if not continue
                    let input: f64 = match line[1].parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };           
                    
                    input_vec.push(input);
                    output_vec.push(output);
                }
            }

            return DataSet {
                size: input_vec.len(),
                input: input_vec,
                output: output_vec,
            };
        }

        return DataSet {
            input: Vec::new(),
            output: Vec::new(),
            size: 0,
        };
        
    }

    fn push(&mut self, input: f64, output: f64) {
        self.input.push(input);
        self.output.push(output);
        self.size += 1;
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}