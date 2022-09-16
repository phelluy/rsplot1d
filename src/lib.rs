//! Simple Rust function for plotting 1d curves.
//! It relies on a working install of Python3 and the matplotlib library.
//! # Example
//! ```
//! use rsplot1d::plot1d;
//! let nx = 1000;
//! let h = 2. * std::f64::consts::PI / nx as f64;
//! 
//! let xi : Vec<f64> = (0..nx+1).map(|i| i as f64 * h).collect();
//! 
//! let sxi = xi.iter().map(|x| x.sin()).collect();
//! let cxi = xi.iter().map(|x| x.cos()).collect();
//! 
//! plot1d(&xi, &sxi, &cxi); 
//! ```
//! 

// cette fonction permet de tracer les vecteurs y et z en fonction de x
// mettre 2 fois y si on ne veut tracer que y
pub fn plot1d(x: &Vec<f64>, y: &Vec<f64>, z: &Vec<f64>) {
    writepy();
    use std::fs::File;
    use std::io::BufWriter;
    use std::io::Write;
    let filename = "rsplot1d.dat";
    {
        let meshfile = File::create(filename).unwrap();
        let mut meshfile = BufWriter::new(meshfile); // create a buffer for faster writes...

        x.iter()
            .zip(y.iter().zip(z.iter()))
            .for_each(|(x, (y, z))| {
                writeln!(meshfile, "{} {} {}", *x, *y, *z).unwrap();
            });
    }

    use std::process::Command;
    Command::new("python3")
        .arg("rsplot1d.py")
        .status()
        .expect("plot failed: you probably need to install python3 and matplotlib");
}

// cette fonction écrit le script python qui va tracer le graphe
fn writepy() {
    let pycom = r#"
from matplotlib.pyplot import *
#from math import *

with open("ploplo.dat", "r") as f:
    contenu = f.read().split()

#print(contenu)
np = len(contenu)//3
#print(np)

x = [float(contenu[3*i]) for i in range(np)]
y = [float(contenu[3*i+1]) for i in range(np)]
z = [float(contenu[3*i+2]) for i in range(np)]
plot(x, y, color="blue")
plot(x, z, color="red")
show()
"#;

    use std::fs::File;
    use std::io::BufWriter;
    use std::io::Write;

    let meshfile = File::create("rsplot1d.py").unwrap();
    let mut meshfile = BufWriter::new(meshfile); // create a buffer for faster writes...
    writeln!(meshfile, "{}", pycom).unwrap();
}