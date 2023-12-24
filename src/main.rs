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
            epochs: 100,
            learning_rate: 0.000001,
        },
    );

    let _ = model.fit();

    println!("b: {}", model.b);
    println!("w: {}", model.w);
    println!("cost: {}", model.cost);
    println!("predict: {:?}", model.predict(1.0)); 

}
