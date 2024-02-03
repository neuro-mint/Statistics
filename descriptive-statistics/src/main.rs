
fn main() {
    let n = generate_data(1000);
    descriptive_statistics(&n);
}

use rand::Rng;
fn generate_data(length: i32) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut n= Vec::new();
    
    for _i in 0..length {
        n.push(rng.gen_range(0.0..1.5e+4));
    }
    let n = n.to_vec();
    return n;
}

fn descriptive_statistics(n: &Vec<f64>) {
    let mean: f64 = simple_mean(n);
    let median: f64 = median(n);
    let mode: f64 = mode(n);
    let sd: f64 = standard_deviation(n, mean);
    let deviations: (f64, f64) = mean_deviation(&n, mean, median);

    println!("");
    println!("🗇 Measures of central tendency ");
    println!("≫ Mean   = {}", simple_mean(&n));
    println!("≫ Standard error  = {}", standard_error(&n, sd));
    println!("≫ Median = {}", median);
    println!("≫ Mode   = {}", mode);
    println!("≫ {}", symmetrical_distrubution(mean, median, mode));

    println!("");
    println!("🗇 Measures of variablity");
    println!("≫  Range    = {}", range(&n));
    println!("≫  Variance = {}", variance(&n, mean));
    println!("≫  Standard deviation = {}", standard_deviation(&n, mean));
    println!("≫  Absolute deviation (mean)   = {}", deviations.0);
    println!("≫  Absolute deviation (median) = {}", deviations.1);

    println!("");
    println!("🗇 Central moments");
    println!("≫  First  moment  = {}", central_moment(&n, mean, 1));
    println!("≫  Second moment  = {}", central_moment(&n, mean, 2));
    println!("≫  Third  moment  = {}", central_moment(&n, mean, 3));
    println!("≫  Fourth moment  = {}", central_moment(&n, mean, 4));
    println!("≫  Fifth  moment  = {}", central_moment(&n, mean, 5));

}



fn simple_mean(n: &Vec<f64>) -> f64 { 
    let mut mean: f64 = 0.0;
    for i in n {
        mean = mean + i ;
    } 
    mean = mean/(n.len() as f64);
    return mean;
}

fn standard_error(n: &Vec<f64>, sd: f64) -> f64 {
    let std_error: f64 = sd/n.len().pow(1/2) as f64;
    return std_error;
}

/* Geometric & Hearmonic means
fn geometric_mean(n: &Vec<f64>) -> f64 {
    let mut mean: f64 = 1.0;
    for i in n {
        mean = mean * i ;
    } 
    mean = mean.powf(1.0/(n.len() as f64));
    return mean;
}
fn harmonic_mean(n: &Vec<f64>) -> f64 {
    let mut denominator: f64 = 0.0;
    for i in n {denominator = denominator + 1.0/i;} 
    let mean: f64 = (n.len() as f64)/denominator;
    return mean;
}
*/

fn median(n: &Vec<f64>) -> f64 {
    let mut n: Vec<f64> = n.to_vec();
    n.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match n.len() as f64 % 2.0 == 0.0 {
        true => {
            let median = (n[n.len()/2] + n[n.len()/2 + 1] )/2.0;
            return median;
        }
        false => {
            let median = n[(n.len()+1)/2] + 0.5;
            return median;
        }
    }
}

fn mode(n: &Vec<f64>) -> f64 {
    n.to_vec(); 
    let mut mode = 0.0;
    let mut unique_values: Vec<f64> = Vec::new();
    let mut frequencies: Vec<usize> = Vec::new();
    for i in 0..n.len() {
        match unique_values.contains(&n[i]) {
            true => continue, 
            false => unique_values.push(n[i]),
        } 
    }
    for i in &unique_values {
        frequencies.push(n.iter().filter(|&n| n == i).count().into());
    }
    let max_freq = frequencies.iter().max().unwrap();
    for i in 0..unique_values.len() {
        if frequencies[i] == *max_freq  {
            mode = unique_values[i] as f64;
        } else {
            continue;
        }
    }
    return mode;
}

fn range(n: &Vec<f64>) -> f64 {
    let mut n: Vec<f64> = n.to_vec();
    n.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let range = n[n.len()-1] - n[0];
    return range; 
}   

fn standard_deviation(n: &Vec<f64>, mean: f64) -> f64 {
    let mut sum: f64 = 0.0;
    for i in n.to_vec() {sum = sum + (i - mean).powi(2)}
    let sd: f64 = (sum/((n.len() as f64)-1.0)).powf(1.0/2.0);
    return sd;
}

fn variance(n: &Vec<f64>, mean: f64) -> f64 {
    let mut sum: f64 = 0.0;
    for i in n.to_vec() {sum = sum + (i - mean).powi(2)}
    let variance: f64 = sum/n.len() as f64;
    return variance;
}

fn mean_deviation(n: &Vec<f64>, mean: f64, median: f64) -> (f64, f64) {
    let mut sum: f64 = 0.0;
    for i in n.to_vec() {sum = sum + (i - mean)};
    let absolute_deviation_about_mean: f64 = sum/n.len() as f64;

    let mut sum: f64 = 0.0;
    for i in n.to_vec() {sum = sum + (i - median)};
    let absolute_deviation_about_median: f64 = sum/n.len() as f64;

    return (absolute_deviation_about_mean, absolute_deviation_about_median);
}

fn symmetrical_distrubution(mean: f64, median: f64, mode: f64) -> String {
    match mean == median && mean == mode {
        true => return "The data is a symmetrical distrubution.".to_owned(),
        false => return "Not a symmetrical distrubution.".to_owned(),
    }
}

fn central_moment(n: &Vec<f64>, mean: f64, moment_number: i32) -> f64 {
    let mut sum: f64 = 0.0;
    for i in n.to_vec() {sum = sum + (i - mean).powi(moment_number)}
    let moment: f64 = sum/(n.len() as f64);
    return moment;
}
