use crate::libs::dataset::lineal_dataset::DataSet;
use super::gradient::{calculate_cost, gradient_descent};

use super::model::Model;

pub struct LinealRegressionOptions {
    pub epochs: u32,
    pub learning_rate: f64,
}

pub struct LinealRegression {
    pub b: f64,
    pub w: f64,
    pub cost: f64,
    pub training_data: DataSet,
    pub options: LinealRegressionOptions,
}

impl LinealRegression {
  /// Creates a new instance of a linear regression model.
  ///
  /// # Arguments
  ///
  /// * `training_data`: The `DataSet` containing the training data to be used for model fitting.
  /// * `options`: The `LinealRegressionOptions` specifying configuration options for the model.
  ///
  /// # Returns
  ///
  /// A new `LinealRegression` instance with initial model parameters set to 0.0.
  pub fn new(training_data: DataSet, options: LinealRegressionOptions) -> Self {
    // Initialize model parameters and store training data and options.
    Self {
        b: 0.0,      // Initial bias term
        w: 0.0,      // Initial weight term
        cost: 0.0,   // Initial cost
        training_data,
        options,
    }
  }
}

impl Model<f64, f64> for LinealRegression {
    fn fit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
      
      let (b, w) = gradient_descent(self.options.epochs, self.options.learning_rate, &self.b, &self.w, &self.training_data);
      
      self.b = b;
      self.w = w;
      self.cost = calculate_cost(&self.b, &self.w, &self.training_data);
      
      Ok(())
    }

    fn predict(&mut self, value: f64) -> Result<f64, Box<dyn std::error::Error>> { 
        // Perform necessary model-specific calculations
        let prediction = self
            .w
            .mul_add(value, self.b);

        Ok(prediction)
    }
}
