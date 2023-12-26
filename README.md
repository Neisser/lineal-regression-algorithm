# Linear Regression Project

## Overview

This project implements a linear regression model in Rust, capable of fitting a line to a set of data points and making predictions based on the learned model.

## Key Features

- **Dataset loading:** Loads data from CSV files using a custom `DataSet` struct.
- **Model training:** Trains a linear regression model using gradient descent with configurable options for epochs and learning rate.
- **Prediction:** Uses the trained model to make predictions on new input data.

## Usage

1. **Ensure Rust is installed:** Follow the official Rust installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. **Clone this repository:** `git clone https://github.com/your-username/linear-regression-project`
3. **Navigate to the project directory:** `cd linear-regression-project`
4. **Run the project:** `cargo run`

## Project Structure

- **src/main.rs:** Contains the main entry point of the project.
- **src/libs/dataset/lineal_dataset.rs:** Defines the `DataSet` struct for handling data.
- **src/libs/newton/lineal_regression.rs:** Implements the `LinealRegression` model and training logic.
- **src/libs/newton/model.rs:** Contains a generic `Model` trait for machine learning models.

## Configuration

- **data.csv:** Stores the training data in CSV format.
- **test.csv:** Stores the test data for evaluation.
- **LinealRegressionOptions** in `main.rs` allows adjusting training parameters.

## Additional Information

- **README.md:** This file (provides more detailed information about the project).

## Contributing

We welcome contributions! Please feel free to open issues or pull requests to suggest improvements or fix any issues.

## License

This project is licensed under the MIT License.
