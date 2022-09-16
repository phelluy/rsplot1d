
fn main() {
    use rsplot1d::plot1d;

    let nx = 1000;
    let h = 2. * std::f64::consts::PI / nx as f64;

    let xi : Vec<f64> = (0..nx+1).map(|i| i as f64 * h).collect();

    let sxi = xi.iter().map(|x| x.sin()).collect();
    let cxi = xi.iter().map(|x| x.cos()).collect();

    plot1d(&xi, &sxi, &cxi);

}