use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
struct TrainingSet {
    input: Vec<f64>,
    output: Vec<f64>,
    size: usize,
}

#[allow(dead_code)]
impl TrainingSet {
    fn new() -> TrainingSet {
        TrainingSet {
            input: Vec::new(),
            output: Vec::new(),
            size: 0,
        }
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

#[allow(dead_code)]
fn main() {
    println!("Hello, Linea Regression!");

    // Create a path to the desired file
    let path = Path::new("data.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // create and empty vector
    let mut data: TrainingSet = TrainingSet::new();
    
    // Read the file contents into a string, returns `io::Result<usize>`
    // let mut content = String::new();
    // match file.read_to_string(&mut content) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, content),
    // }

    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{:?}", ip);

                let line: Vec<&str> = ip.split(",").collect();

                // set input and validate if it is a number, if not continue
                let input: f64 = match line[0].parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let output: f64 = match line[1].parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                
                data.push(input, output);
                
            }
        }
    }

    println!("{:?}", data)



    // let file: File = File::open(path).unwrap();
    // let reader: BufReader<File> = BufReader::new(file);

    

    // parse file
    // for line in reader.read_line() {
    //     let line: String = line.unwrap();
    //     let line: Vec<&str> = line.split(",").collect();

    //     for value in line {
    //         let value: f64 = value.parse().unwrap();
    //         data.push(value);
    //     }
    // }

    // train model

    // test model
}