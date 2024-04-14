use colours::DEFAULT_COLOURS;
use mandelbrot_explorer::colours;
use mandelbrot_explorer::renderer::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn simd_test() {
    mandelbrot_xy_coords_from_params_simd(
        num::Complex::new(0.0, 0.0),
        0.0009765625 * 8.0,
        1000,
        500,
        500,
        &DEFAULT_COLOURS,
    );
}

#[divan::bench]
fn simd_parallel_test() {
    mandelbrot_from_params_simd_parallel(
        num::Complex::new(0.0, 0.0),
        0.0009765625 * 8.0,
        1000,
        500,
        500,
        &DEFAULT_COLOURS,
    );
}

#[divan::bench]
fn standard_test() {
    mandelbrot_xy_coords_from_params(
        num::Complex::new(0.0, 0.0),
        0.0009765625 * 8.0,
        1000,
        500,
        500,
        &DEFAULT_COLOURS,
    );
}

#[divan::bench]
fn standard_parallel_test() {
    mandelbrot_from_params_parallel(
        num::Complex::new(0.0, 0.0),
        0.0009765625 * 8.0,
        1000,
        500,
        500,
        &DEFAULT_COLOURS,
    );
}
