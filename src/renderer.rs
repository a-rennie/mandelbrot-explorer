use crate::backend::MandelbrotPlane;
use num::complex::ComplexFloat;

type RawMandelbrotColours = Vec<((u64, u64), (u8, u8, u8))>;

// plot all points from a MandelbrotPlane to xy coordinates on an image and rgb colouring
pub fn mandelbrot_xy_coordinates_with_colours(
    set: MandelbrotPlane,
    colours: &[(u8, u8, u8)],
) -> RawMandelbrotColours {
    let points = set.points_with_colours(colours);
    points
        .into_iter()
        .map(|(point, colour)| {
            (
                (
                    ((point.point().re() - set.re_min())
                        / ((set.re_max() - set.re_min()) / (set.width() as f64)))
                        .round() as u64,
                    ((point.point().im() - set.im_min())
                        / ((set.im_max() - set.im_min()) / (set.height() as f64)))
                        .round() as u64,
                ),
                colour,
            )
        })
        .collect()
}

// plot points but in parallel
pub fn mandelbrot_xy_coordinates_with_colours_parallel(
    set: MandelbrotPlane,
    colours: &[(u8, u8, u8)],
) -> RawMandelbrotColours {
    let points = set.points_with_colours_parallel(colours);
    points
        .into_iter()
        .map(|(point, colour)| {
            (
                (
                    ((point.point().re() - set.re_min())
                        / ((set.re_max() - set.re_min()) / (set.width() as f64)))
                        .round() as u64,
                    ((point.point().im() - set.im_min())
                        / ((set.im_max() - set.im_min()) / (set.height() as f64)))
                        .round() as u64,
                ),
                colour,
            )
        })
        .collect()
}

pub fn mandelbrot_xy_coordinates_with_colours_simd(
    set: MandelbrotPlane,
    colours: &[(u8, u8, u8)],
) -> RawMandelbrotColours {
    let points = set.points_with_colours_simd(colours);
    points
        .into_iter()
        .map(|(point, colour)| {
            (
                (
                    ((point.point().re() - set.re_min())
                        / ((set.re_max() - set.re_min()) / (set.width() as f64)))
                        .round() as u64,
                    ((point.point().im() - set.im_min())
                        / ((set.im_max() - set.im_min()) / (set.height() as f64)))
                        .round() as u64,
                ),
                colour,
            )
        })
        .collect()
}

pub fn mandelbrot_xy_coords_colours_simd_parallel(
    set: MandelbrotPlane,
    colours: &[(u8, u8, u8)],
) -> RawMandelbrotColours {
    let points = set.points_with_colours_simd_parallel(colours);
    points
        .into_iter()
        .map(|(point, colour)| {
            (
                (
                    ((point.point().re() - set.re_min())
                        / ((set.re_max() - set.re_min()) / (set.width() as f64)))
                        .round() as u64,
                    ((point.point().im() - set.im_min())
                        / ((set.im_max() - set.im_min()) / (set.height() as f64)))
                        .round() as u64,
                ),
                colour,
            )
        })
        .collect()
}

// instead of using a MandelbrotPlane,
// use simpler parameters which then get converted into a MandelbrotPlane
pub fn mandelbrot_xy_coords_from_params(
    centre: num::Complex<f64>,
    resolution: f64,
    max_iterations: u64,
    width: u64,
    height: u64,
    colours: &[(u8, u8, u8)],
) -> RawMandelbrotColours {
    let real_width = width as f64 * resolution;
    let real_height = height as f64 * resolution;
    let re_max = centre.re() + real_width / 2.0;
    let re_min = centre.re() - real_width / 2.0;
    let im_max = centre.im() + real_height / 2.0;
    let im_min = centre.im() - real_height / 2.0;

    mandelbrot_xy_coordinates_with_colours(
        MandelbrotPlane::new(
            re_min,
            re_max,
            im_min,
            im_max,
            width,
            height,
            max_iterations,
        ),
        colours,
    )
}

// render with parameters, but in parallel
pub fn mandelbrot_from_params_parallel(
    centre: num::Complex<f64>,
    resolution: f64,
    max_iterations: u64,
    width: u64,
    height: u64,
    colours: &[(u8, u8, u8)],
) -> RawMandelbrotColours {
    let real_width = width as f64 * resolution;
    let real_height = height as f64 * resolution;
    let re_max = centre.re() + real_width / 2.0;
    let re_min = centre.re() - real_width / 2.0;
    let im_max = centre.im() + real_height / 2.0;
    let im_min = centre.im() - real_height / 2.0;

    mandelbrot_xy_coordinates_with_colours_parallel(
        MandelbrotPlane::new(
            re_min,
            re_max,
            im_min,
            im_max,
            width,
            height,
            max_iterations,
        ),
        colours,
    )
}

pub fn mandelbrot_xy_coords_from_params_simd(
    centre: num::Complex<f64>,
    resolution: f64,
    max_iterations: u64,
    width: u64,
    height: u64,
    colours: &[(u8, u8, u8)],
) -> RawMandelbrotColours {
    let real_width = width as f64 * resolution;
    let real_height = height as f64 * resolution;
    let re_max = centre.re() + real_width / 2.0;
    let re_min = centre.re() - real_width / 2.0;
    let im_max = centre.im() + real_height / 2.0;
    let im_min = centre.im() - real_height / 2.0;

    mandelbrot_xy_coordinates_with_colours_simd(
        MandelbrotPlane::new(
            re_min,
            re_max,
            im_min,
            im_max,
            width,
            height,
            max_iterations,
        ),
        colours,
    )
}

pub fn mandelbrot_from_params_simd_parallel(
    centre: num::Complex<f64>,
    resolution: f64,
    max_iterations: u64,
    width: u64,
    height: u64,
    colours: &[(u8, u8, u8)],
) -> RawMandelbrotColours {
    let real_width = width as f64 * resolution;
    let real_height = height as f64 * resolution;
    let re_max = centre.re() + real_width / 2.0;
    let re_min = centre.re() - real_width / 2.0;
    let im_max = centre.im() + real_height / 2.0;
    let im_min = centre.im() - real_height / 2.0;

    mandelbrot_xy_coords_colours_simd_parallel(
        MandelbrotPlane::new(
            re_min,
            re_max,
            im_min,
            im_max,
            width,
            height,
            max_iterations,
        ),
        colours,
    )
}
