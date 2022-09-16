# rsplot1d
Simple utility for plotting 1d curves in Rust. It generates a Python script based on matplotlib and launches it.

Example that plots the $\sin$ and $\cos$ function on the interval $[0,2 \pi]$:

```rust
    use rsplot1d::plot1d;

    let nx = 1000;
    let h = 2. * std::f64::consts::PI / nx as f64;

    let xi : Vec<f64> = (0..nx+1).map(|i| i as f64 * h).collect();

    let sxi = xi.iter().map(|x| x.sin()).collect();
    let cxi = xi.iter().map(|x| x.cos()).collect();

    plot1d(&xi, &sxi, &cxi);
```

Don't forget to add the line

```
rsplot1d = {git = "https://github.com/phelluy/rsplot1d"}
```

in the `[dependencies]` section of your `Cargo.toml`.
