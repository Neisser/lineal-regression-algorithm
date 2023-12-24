use crate::libs::dataset::lineal_dataset::DataSet;

pub fn calculate_cost(b: &f64, w: &f64, data: &DataSet) -> f64 {
  let sum: f64 = data.input.iter()
  .zip(data.output.iter())
  .map(|(x, y)| (b + (w * x) - y).powf(2.0)).sum();

  // println!("Sum: {}", sum);
  // println!("Size: {}", data.size);

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




    // train model
    // let mut alpha: f64 = 0.000001;
    // let epochs: u32 = 100;
    // let mut theta_0: f64 = 0f64;
    // let mut theta_1: f64 = 0.0;

    // for i in 0..1 {
    //     alpha = alpha * 0.005_f64.powf(i as f64);

    //     let (t0, t1) = gradient_descent(epochs, alpha, &theta_0, &theta_1, &data);

    //     theta_0 = t0;
    //     theta_1 = t1;

    //     let cost: f64 = calculate_cost(&t0, &t1, &data);
    //     print!("Epoch: {} ", i);
    //     println!("Cost: {}", cost);
    //     println!("Theta 0: {}", t0);
    //     println!("Theta 1: {}", t1);
    //     println!("******************************************************************");
    // }