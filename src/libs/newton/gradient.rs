use crate::libs::dataset::lineal_dataset::DataSet;

/// Calculates the mean squared error (MSE) of a linear regression model on a given dataset.

/// Args:
/// * `b`: The bias term of the model.
/// * `w`: The weight vector of the model.
/// * `data`: A DataSet object containing the input and output data points.

/// Returns:
/// The mean squared error of the model on the given dataset.

/// Example usage
/// ```rust
/// let input_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let output_data = vec![2.0, 4.0, 6.0, 8.0, 10.0];
/// let data = DataSet::new(input_data, output_data);
///
/// let b = 1.0;
/// let w = 2.0;
///
/// let cost = calculate_cost(&b, &w, &data);
///
/// println!("Mean squared error: {}", cost);
/// ```

pub fn calculate_cost(b: &f64, w: &f64, data: &DataSet) -> f64 {
  // Calculate squared errors for each data point
  let squared_errors: f64 = data.output.iter()
      .zip(data.input.iter())
      .map(|(x, y)| (b + (w * x) - y).powf(2.0))
      .sum();

  // Handle potential division by zero
  if data.size == 0 {
      return f64::NAN; // Or return a default value or panic, depending on your error handling strategy
  }

  // Calculate mean squared error
  return squared_errors / (2.0 * data.size as f64);
}

pub fn gradient_descent(epochs: u32, alpha: f64, b: &f64, w: &f64, data: &DataSet) -> (f64, f64) {
  let mut theta_0: f64 = *b;
  let mut theta_1: f64 = *w;

  for _ in 0..epochs {
      let m: f64 = data.size as f64;

      let theta_0_temp: f64 = data.output
          .iter()
          .zip(data.input.iter())
          .map(|(x, y)| (theta_0 + (theta_1 * x) - y))
          .sum();

      let theta_1_temp: f64 = data.output
          .iter()
          .zip(data.input.iter())
          .map(|(x, y)| (theta_0 + (theta_1 * x) - y) * x)
          .sum();

      theta_0 -= alpha * (theta_0_temp / m);
      theta_1 -= alpha * (theta_1_temp / m);
  }
  return (theta_0, theta_1)
}