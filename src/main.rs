mod libs;

use std::path::Path;

use libs::dataset::lineal_dataset::DataSet;
use libs::newton::lineal_regression::{LinealRegression, LinealRegressionOptions};
use libs::newton::model::Model;

#[allow(dead_code)]
fn main() {
    println!("Hello, Lineal Regression!");

    let path: &Path = Path::new("data.csv");

    let data: DataSet = DataSet::new(path);
    
    println!("{:?}", data);

    let mut model = LinealRegression::new(
        data,
        LinealRegressionOptions {
            epochs: 1000,
            learning_rate: 0.01,
        },
    );

    let _ = model.fit();

    println!("b: {}", model.b);
    println!("w: {}", model.w);
    println!("cost: {}", model.cost);

    let test_path: &Path = Path::new("test.csv");

    let test_data: DataSet = DataSet::new(test_path);

    let test_inputs = test_data.input;

    for value in test_inputs {
        println!("predict {}: {:?}", value, model.predict(value).unwrap()); 
    }

}
