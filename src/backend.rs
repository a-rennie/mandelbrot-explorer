use num::complex::ComplexFloat;
use num::Complex;
use rayon::prelude::*;
use std::simd::prelude::*;
use std::sync::{Arc, Mutex};
// A point on the complex plane
#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn points_with_iterations_simd(self) -> Vec<(MandelbrotPoint, u64)> {
        let mut re_points = Vec::new();
        let mut im_points = Vec::new();
        let mut points_out: Vec<(MandelbrotPoint, u64)> = Vec::new();
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
                re_points.push(point.point.re());
                im_points.push(point.point.im());
            }
        }
        let mut re_simd = f64x8::splat(0.0);
        let mut im_simd = f64x8::splat(0.0);
        let mut queue = std::iter::zip(re_points, im_points);
        let mut z_re = f64x8::splat(0.0);
        let mut z_im = f64x8::splat(0.0);
        let mut mask = mask64x8::splat(false);
        let mut iterations = u64x8::splat(0);
        //let mut index = f64x64::from_slice(&(0..64).map(|x| x as f64).collect::<Vec<_>>()[..]);
        'outer: loop {
            let mask_indexable = mask.to_array();
            for i in 0usize..8 {
                // let next = queue.next();
                // if next.is_none() && !mask.any() {
                //     break 'outer;
                // }
                if !mask_indexable[i] {
                    let next = queue.next();
                    match next {
                        Some(item) => {
                            re_simd[i] = item.0;
                            im_simd[i] = item.1;
                            iterations[i] = 0;
                            mask.set(i, true);
                            z_re[i] = 0.0;
                            z_im[i] = 0.0;
                        }
                        None => {
                            if !(mask.any()) {
                                break 'outer;
                            }
                        }
                    }
                }
            }
            for _ in 0..50 {
                (z_re, z_im) = (mask.select(((z_re * z_re) - (z_im * z_im)) + re_simd, z_re), mask.select((f64x8::splat(2.0) * (z_re * z_im)) + im_simd, z_im));
                mask = ((z_re * z_re) + (z_im * z_im)).simd_le(f64x8::splat(4.0));
                iterations = mask.select(iterations + u64x8::splat(1), iterations);
                if !(mask.any()) {
                    break;
                }
            }
            mask &= iterations.simd_lt(u64x8::splat(self.max_iterations));
            let mask_indexable = mask.to_array();
            for i in 0usize..8 {
                if !mask_indexable[i] {
                    points_out.push((
                        MandelbrotPoint::new(Complex::new(re_simd[i], im_simd[i])),
                        iterations[i],
                    ));
                }
            }
        }
        points_out
    }

    pub fn points_with_iterations_simd_parallel(self) -> Vec<(MandelbrotPoint, u64)> {
        let mut re_points = Vec::new();
        let mut im_points = Vec::new();
        let points_out: Arc<Mutex<Vec<(MandelbrotPoint, u64)>>> = Arc::new(Mutex::new(Vec::new()));
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
                re_points.push(point.point.re());
                im_points.push(point.point.im());
            }
        }
        let queue = Arc::new(Mutex::new(std::iter::zip(re_points, im_points)));

        (0..10).into_par_iter().for_each(|_| {

            let mut re_simd = f64x8::splat(0.0);
            let mut im_simd = f64x8::splat(0.0);
            let mut z_re = f64x8::splat(0.0);
            let mut z_im = f64x8::splat(0.0);
            let mut mask = mask64x8::splat(false);
            let mut iterations = u64x8::splat(0);
            //let mut index = f64x64::from_slice(&(0..64).map(|x| x as f64).collect::<Vec<_>>()[..]);
            'outer: loop {
                let mask_indexable = mask.to_array();
                for i in 0usize..8 {
                    // let next = queue.next();
                    // if next.is_none() && !mask.any() {
                    //     break 'outer;
                    // }
                    if !mask_indexable[i] {
                        let next = queue.lock().unwrap().next();
                        match next {
                            Some(item) => {
                                re_simd[i] = item.0;
                                im_simd[i] = item.1;
                                iterations[i] = 0;
                                mask.set(i, true);
                                z_re[i] = 0.0;
                                z_im[i] = 0.0;
                            }
                            None => {
                                if !(mask.any()) {
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                for _ in 0..50 {
                    (z_re, z_im) = (mask.select(((z_re * z_re) - (z_im * z_im)) + re_simd, z_re), mask.select((f64x8::splat(2.0) * (z_re * z_im)) + im_simd, z_im));
                    mask = ((z_re * z_re) + (z_im * z_im)).simd_le(f64x8::splat(4.0));
                    iterations = mask.select(iterations + u64x8::splat(1), iterations);
                    if !(mask.any()) {
                        break;
                    }
                }
                mask &= iterations.simd_lt(u64x8::splat(self.max_iterations));
                let mask_indexable = mask.to_array();
                for i in 0usize..8 {
                    if !mask_indexable[i] {
                        points_out.lock().unwrap().push((
                            MandelbrotPoint::new(Complex::new(re_simd[i], im_simd[i])),
                            iterations[i],
                        ));
                    }
                }
            }});
        let x = points_out.lock().unwrap().clone(); x
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
    pub fn points_with_colours(
        self,
        colours: &[(u8, u8, u8)],
    ) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        self.points_with_iterations()
            .into_iter()
            .map(|(point, iterations)| {
                let i = iterations;
                let point_coord = point.point;
                let smoothed = point_coord.norm().log2();
                let colour_i: usize =
                    ((i as f64 + 10.0 - smoothed).sqrt() * 256.0).round() as usize % colours.len();
                (
                    point,
                    if self.max_iterations == i {
                        (0, 0, 0)
                    } else {
                        colours[colour_i]
                    },
                )
            })
            .collect()
    }

    // same but in parallel
    pub fn points_with_colours_parallel(
        self,
        colours: &[(u8, u8, u8)],
    ) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        self.points_with_iterations_parallel()
            .into_par_iter()
            .map(|(point, iterations)| {
                let i = iterations;
                let point_coord = point.point;
                let smoothed = point_coord.norm().log2();
                let colour_i: usize =
                    ((i as f64 + 10.0 - smoothed).sqrt() * 255.0).round() as usize % colours.len();
                (
                    point,
                    if self.max_iterations == i {
                        (0, 0, 0)
                    } else {
                        colours[colour_i]
                    },
                )
            })
            .collect()
    }

    pub fn points_with_colours_simd(
        self,
        colours: &[(u8, u8, u8)],
    ) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        self.points_with_iterations_simd()
            .into_iter()
            .map(|(point, iterations)| {
                let i = iterations;
                let point_coord = point.point;
                let smoothed = point_coord.norm().log2();
                let colour_i: usize =
                    ((i as f64 + 10.0 - smoothed).sqrt() * 256.0).round() as usize % colours.len();
                (
                    point,
                    if self.max_iterations == i {
                        (0, 0, 0)
                    } else {
                        colours[colour_i]
                    },
                )
            })
            .collect()

    }
    pub fn points_with_colours_simd_parallel(
        self,
        colours: &[(u8, u8, u8)],
    ) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        self.points_with_iterations_simd_parallel()
            .into_par_iter()
            .map(|(point, iterations)| {
                let i = iterations;
                let point_coord = point.point;
                let smoothed = point_coord.norm().log2();
                let colour_i: usize =
                    ((i as f64 + 10.0 - smoothed).sqrt() * 255.0).round() as usize % colours.len();
                (
                    point,
                    if self.max_iterations == i {
                        (0, 0, 0)
                    } else {
                        colours[colour_i]
                    },
                )
            })
            .collect()
    }
}