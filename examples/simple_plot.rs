use rsplot1d::plot1d;

fn main() {
    let nx = 1000;
    let h = 1. / nx as f64;

    let xi : Vec<f64> = (0..nx).map(|i| i as f64 * h + h / 2.).collect();

    let pi = std::f64::consts::PI;
    let sxi = xi.iter().map(|x| (2.*pi*x).sin()).collect();
    let cxi = xi.iter().map(|x| (2.*pi*x).cos()).collect();

    plot1d(&xi, &sxi, &cxi);

}