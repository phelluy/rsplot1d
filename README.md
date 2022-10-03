# rsplot1d
Simple utility for plotting 1d curves in Rust.

It generate a data file `rsplot1d.dat`
and a Python script `rsplot1d.py` for plotting the data.

It also launches automatically the Python script.

It requires a working installation of Python3 and matplotlib.

Example that plots the $\sin$ and $\cos$ function on the interval $[0,2 \pi]$:

```rust
    use rsplot1d::plot1d;
    use rsplot1d::plot;

    let nx = 1000;
    let h = 2. * std::f64::consts::PI / nx as f64;

    let xi : Vec<f64> = (0..nx+1).map(|i| i as f64 * h).collect();

    let sxi = xi.iter().map(|x| x.sin()).collect();
    let cxi = xi.iter().map(|x| x.cos()).collect();

    plot1d(&xi, &sxi, &cxi);
    plot1d(&xi[0..nx+1], &sxi[0..nx+1], &cxi[0..nx+1]);    
```

Don't forget to add the line

```
rsplot1d = {git = "https://github.com/phelluy/rsplot1d"}
```

in the `[dependencies]` section of your `Cargo.toml`.
