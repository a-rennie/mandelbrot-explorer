use crate::backend;
use crate::backend::MandelbrotPlane;
use num::complex::ComplexFloat;
use rayon::prelude::*;

pub fn mandelbrot_xy_coordinates_with_colours(
    set: MandelbrotPlane,
) -> Vec<((u64, u64), (u8, u8, u8))> {
    let mut points = set.points_with_colours();
    let points = points
        .iter()
        .map(|point| {
            (
                (
                    ((point.0.point().re() - set.re_min())
                        / ((set.re_max() - set.re_min()) / (set.width() as f64)))
                        .round() as u64,
                    ((point.0.point().im() - set.im_min())
                        / ((set.im_max() - set.im_min()) / (set.height() as f64)))
                        .round() as u64,
                ),
                point.1,
            )
        })
        .collect();

    points
}

pub fn mandelbrot_xy_coordinates_with_colours_parallel(
    set: MandelbrotPlane,
) -> Vec<((u64, u64), (u8, u8, u8))> {
    let mut points = set.points_with_colours_parallel();
    let points = points
        .iter()
        .map(|point| {
            (
                (
                    ((point.0.point().re() - set.re_min())
                        / ((set.re_max() - set.re_min()) / (set.width() as f64)))
                        .round() as u64,
                    ((point.0.point().im() - set.im_min())
                        / ((set.im_max() - set.im_min()) / (set.height() as f64)))
                        .round() as u64,
                ),
                point.1,
            )
        })
        .collect();

    points
}
pub fn mandelbrot_xy_coords_from_params(
    centre: num::Complex<f64>,
    resolution: f64,
    max_iterations: u64,
    width: u64,
    height: u64,
) -> Vec<((u64, u64), (u8, u8, u8))> {
    let real_width = width as f64 * resolution;
    let real_height = height as f64 * resolution;
    let re_max = centre.re() + real_width / 2.0;
    let re_min = centre.re() - real_width / 2.0;
    let im_max = centre.im() + real_height / 2.0;
    let im_min = centre.im() - real_height / 2.0;

    mandelbrot_xy_coordinates_with_colours(MandelbrotPlane::new(
        re_min,
        re_max,
        im_min,
        im_max,
        width,
        height,
        max_iterations,
    ))
}

// TODO! sort out histogram colouring from parallel threads
pub fn mandelbrot_from_params_parallel(
    centre: num::Complex<f64>,
    resolution: f64,
    max_iterations: u64,
    width: u64,
    height: u64,
) -> Vec<((u64, u64), (u8, u8, u8))> {
    let real_width = width as f64 * resolution;
    let real_height = height as f64 * resolution;
    let re_max = centre.re() + real_width / 2.0;
    let re_min = centre.re() - real_width / 2.0;
    let im_max = centre.im() + real_height / 2.0;
    let im_min = centre.im() - real_height / 2.0;

    mandelbrot_xy_coordinates_with_colours_parallel(MandelbrotPlane::new(
        re_min,
        re_max,
        im_min,
        im_max,
        width,
        height,
        max_iterations,
    ))
}
