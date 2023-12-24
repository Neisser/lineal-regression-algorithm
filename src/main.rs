use std::path::Path;

mod dataset;
mod gradient;

use dataset::DataSet;
use gradient::{calculate_cost, gradient_descent};




#[allow(dead_code)]
fn main() {
    println!("Hello, Lineal Regression!");

    // Create a path to the desired file
    let path: &Path = Path::new("data.csv");
    let data: DataSet = DataSet::new(path);

    
    println!("{:?}", data);

    // train model
    let mut alpha: f64 = 0.000001;
    let epochs: u32 = 100;
    let mut theta_0: f64 = 0f64;
    let mut  theta_1: f64 = 0.0;
    
    for i in 0..1 {
        alpha = alpha * 0.005_f64.powf(i as f64);
        
        let (t0, t1) = gradient_descent(epochs, alpha, &theta_0, &theta_1, &data);
        
        theta_0 = t0;
        theta_1 = t1;

        let cost: f64 = calculate_cost(&t0, &t1, &data);
        print!("Epoch: {} ", i);
        println!("Cost: {}", cost);
        println!("Theta 0: {}", t0);
        println!("Theta 1: {}", t1);
        println!("******************************************************************");
    }
}