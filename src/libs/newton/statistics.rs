struct Statistics {
    pub mean: f64,
    pub variance: f64,
    pub std_dev: f64,
    pub std_err: f64,
    pub median: f64,
    pub mode: f64,
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub count: usize,
}

impl Statistics {
    pub fn new(data: &Vec<f64>) -> Self {
        let mean: f64 = data.iter().sum::<f64>() / data.len() as f64;
        let variance: f64 = data.iter().map(|x: &f64| (x - mean).powf(2.0)).sum::<f64>() / data.len() as f64;
        let std_dev: f64 = variance.sqrt();
        let std_err: f64 = std_dev / (data.len() as f64).sqrt();
        let median: f64 = median(data);
        let mode: f64 = mode(data);
        let min: f64 = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max: f64 = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let sum: f64 = data.iter().sum::<f64>();
        let count: usize = data.len();

        Self {
            mean,
            variance,
            std_dev,
            std_err,
            median,
            mode,
            min,
            max,
            sum,
            count,
        }
    }
}

fn median(data: &Vec<f64>) -> f64 {
    let mut data: Vec<f64> = data.clone();

    data.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());

    let mid: usize = data.len() / 2;

    if data.len() % 2 == 0 {
        (data[mid] + data[mid - 1]) / 2.0
    } else {
        data[mid]
    }
}

fn mode(data: &Vec<f64>) -> f64 {
    let mut data = data.clone();

    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut mode: f64 = data[0];
    let mut max_count: i32 = 0;
    let mut current: f64 = data[0];
    let mut current_count: i32 = 0;
    
    for &value in data.iter() {
        if value == current {
            current_count += 1;
        } else {
            if current_count > max_count {
                max_count = current_count;
                mode = current;
            }
            current = value;
            current_count = 1;
        }
    }
    mode
}