use num::Complex;
use rayon::prelude::*;
use num::complex::ComplexFloat;

// A point on the complex plane
#[derive(Debug, Copy, Clone)]
pub struct MandelbrotPoint {
    point: Complex<f64>,
}

impl MandelbrotPoint {
    pub fn new(point: Complex<f64>) -> MandelbrotPoint {
        MandelbrotPoint { point }
    }

    pub fn point(self) -> Complex<f64> {
        self.point
    }

    // Return how many iterations it takes for the point to escape a circle of radius 4,
    // and cut off at max_iterations if it does not escape
    pub fn iterations(self, max_iterations: u64) -> u64 {
        let mut iteration = 0;
        let mut z = Complex::new(0.0, 0.0);
        while z.norm_sqr() <= 4.0 && iteration < max_iterations {
            // finding square of distance much faster than sqrt()ing
            z = z * z + self.point;
            iteration += 1
        }
        iteration
    }
}

// A section of the complex plane, the width and height specifying how many individual
// mandelbrot points are rendered in the plane
#[derive(Debug, Copy, Clone)]
pub struct MandelbrotPlane {
    re_min: f64,
    re_max: f64,
    im_min: f64,
    im_max: f64,
    width: u64,
    height: u64,
    max_iterations: u64,
}

impl MandelbrotPlane {
    pub fn new(
        re_min: f64,
        re_max: f64,
        im_min: f64,
        im_max: f64,
        width: u64,
        height: u64,
        max_iterations: u64,
    ) -> MandelbrotPlane {
        MandelbrotPlane {
            re_min,
            re_max,
            im_min,
            im_max,
            width,
            height,
            max_iterations,
        }
    }

    pub fn width(self) -> u64 {
        self.width
    }

    pub fn height(self) -> u64 {
        self.height
    }

    pub fn re_max(self) -> f64 {
        self.re_max
    }

    pub fn re_min(self) -> f64 {
        self.re_min
    }

    pub fn im_max(self) -> f64 {
        self.im_max
    }

    pub fn im_min(self) -> f64 {
        self.im_min
    }

    // Return a Vec with all the mandelbrot points in the plane (width*height),
    // along with the number of iterations (escape time)
    pub fn points_with_iterations(self) -> Vec<(MandelbrotPoint, u64)> {
        let mut points = Vec::new();
        let mut point;
        // create list of all points in bounds specified
        for real in 0..self.width {
            for imaginary in 0..self.height {
                point = MandelbrotPoint::new(Complex::new(
                    (((self.re_max - self.re_min) / (self.width as f64)) * real as f64)
                        + self.re_min,
                    (((self.im_max - self.im_min) / (self.height as f64)) * imaginary as f64)
                        + self.im_min,
                ));
                points.push((point, point.iterations(self.max_iterations)))
            }
        }
        points
    }

    // same, but in parallel :o
    pub fn points_with_iterations_parallel(self) -> Vec<(MandelbrotPoint, u64)> {
        let mut points = Vec::new();
        let mut point;
        // create list of all points in bounds specified
        for real in 0..self.width {
            for imaginary in 0..self.height {
                point = MandelbrotPoint::new(Complex::new(
                    (((self.re_max - self.re_min) / (self.width as f64)) * real as f64)
                        + self.re_min,
                    (((self.im_max - self.im_min) / (self.height as f64)) * imaginary as f64)
                        + self.im_min,
                ));
                points.push(point)
            }
        }
        points
            .into_par_iter()
            .map(|point| (point, point.iterations(self.max_iterations)))
            .collect()
    }

    // Instead of returning points with iterations,
    // this returns the colour for each point as an rgb tuple from a precomputed colour palette
    pub fn points_with_colours(self) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        self.points_with_iterations()
            .iter()
            .map(|point| {
                let i = point.1;
                let point_coord = point.0.point;
                let smoothed = point_coord.norm().log2();
                let colour_i: usize = ((i as f64 + 10.0 - smoothed).sqrt() * 256.0).round()
                    as usize
                    % crate::colours::FRACTAL_COLOURS.len();
                (
                    point.0,
                    if self.max_iterations == i {
                        (0, 0, 0)
                    } else {
                        crate::colours::FRACTAL_COLOURS[colour_i]
                    },
                )
            })
            .collect()
    }

    // same but in parallel
    pub fn points_with_colours_parallel(self) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        self.points_with_iterations_parallel()
            .par_iter()
            .map(|point| {
                let i = point.1;
                let point_coord = point.0.point;
                let smoothed = point_coord.norm().log2();
                let colour_i: usize = ((i as f64 + 10.0 - smoothed).sqrt() * 255.0).round()
                    as usize
                    % crate::colours::FRACTAL_COLOURS.len();
                (
                    point.0,
                    if self.max_iterations == i {
                        (0, 0, 0)
                    } else {
                        crate::colours::FRACTAL_COLOURS[colour_i]
                    },
                )
            })
            .collect()
    }
}
