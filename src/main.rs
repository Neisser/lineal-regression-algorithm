mod libs;

use std::path::Path;

use libs::dataset::lineal_dataset::DataSet;
use libs::newton::lineal_regression::{LinealRegression, LinealRegressionOptions};
use libs::newton::model::Model;

#[allow(dead_code)] // Suppress warnings for unused code (likely for future use)
fn main() {

    // Load data from CSV file
    let path = Path::new("data.csv"); // Create a path to the CSV file
    let data = DataSet::new(path); // Create a DataSet instance from the CSV data
    println!("{:?}", data); // Print a debug representation of the DataSet

    // Create a linear regression model
    let mut model = LinealRegression::new(
        data, // Use the loaded dataset
        LinealRegressionOptions {
            epochs: 1000, // Number of training iterations
            learning_rate: 0.01, // Step size for gradient descent
            nornalize: false, // Normalize the data before training
        },
    );

    // Train the model
    let _ = model.fit(); // Fit the model to the data (discarding the return value)

    // Print model parameters and cost
    println!("b: {}", model.b); // Bias term (intercept)
    println!("w: {}", model.w); // Weight vector (coefficients)
    println!("cost: {}", model.cost); // Final cost (error) after training

    // Load test data
    let test_path = Path::new("test.csv"); // Create a path to the test CSV file
    let test_data = DataSet::new(test_path); // Create a DataSet instance from the test data
    let test_inputs = test_data.input; // Extract input values for prediction

    // Make predictions on test data
    for value in test_inputs {
        println!("predict {}: {:?}", value, model.predict(value).unwrap()); // Predict for each input value
    }

}
