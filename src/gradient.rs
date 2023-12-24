use crate::dataset;

use dataset::DataSet;


pub fn calculate_cost(b: &f64, w: &f64, data: &DataSet) -> f64 {
  let sum: f64 = data.input.iter()
  .zip(data.output.iter())
  .map(|(x, y)| (b + (w * x) - y).powf(2.0)).sum();

  println!("Sum: {}", sum);
  println!("Size: {}", data.size);

  return sum / (2.0 * data.size as f64);
}

pub fn gradient_descent(epochs: u32, alpha: f64, b: &f64, w: &f64, data: &DataSet) -> (f64, f64) {

  let mut theta_0: f64 = 0.0;
  let mut theta_1: f64 = 0.0;

  for _ in 0..epochs {
      let m: f64 = data.size as f64;

      let theta_0_temp: f64 = data.input
          .iter()
          .zip(data.output.iter())
          .map(|(x, y)| (b + (w * x) - y))
          .sum();

      let theta_1_temp: f64 = data.input
          .iter()
          .zip(data.output.iter())
          .map(|(x, y)| (b + (w * x) - y) * x)
          .sum();

      theta_0 = b - alpha * (theta_0_temp / m);
      println!("Theta 0: {}", theta_0);
      theta_1 = w - alpha * (theta_1_temp / m);
      println!("Theta 1: {}", theta_1);
      println!("Cost: {}", calculate_cost(&theta_0, &theta_1, &data));
      println!("---------------------------");
  }
  return (theta_0, theta_1)
}
