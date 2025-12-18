pub fn calculate_mandelbrot(cx: f64, cy: f64, max_iters: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iter = 0;

    while (x * x + y * y <= 4.0) && (iter < max_iters) {
        let x_temp = x * x - y * y + cx;
        y = 2.0 * x * y + cy;
        x = x_temp;
        iter += 1;
    }
    iter
}

pub fn calculate_burning_ship(cx: f64, cy: f64, max_iters: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iter = 0;

    while (x * x + y * y <= 4.0) && (iter < max_iters) {
        let x_temp = x * x - y * y + cx;
        y = 2.0 * x.abs() * y.abs() + cy;
        x = x_temp;
        iter += 1;
    }
    iter
}

pub fn calculate_julia(zx: f64, zy: f64, cx: f64, cy: f64, max_iters: u32) -> u32 {
    let mut x = zx;
    let mut y = zy;
    let mut iter = 0;

    while (x * x + y * y <= 4.0) && (iter < max_iters) {
        let x_temp = x * x - y * y + cx;
        y = 2.0 * x * y + cy;
        x = x_temp;
        iter += 1;
    }
    iter
}