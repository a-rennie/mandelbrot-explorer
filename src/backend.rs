use num::Complex;
use rayon::prelude::*;
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
    pub fn points_with_colours(self) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        let points = self.points_with_iterations();
        let mut points_with_colours: Vec<(MandelbrotPoint, (u8, u8, u8))> = vec![];
        // for (point, iteration) in points {
        //     if iteration < self.max_iterations {
        //         let i = iteration % 16;
        //         let rgb = match i {
        //             0 => (66, 30, 15),
        //             1 => (25, 7, 26),
        //             2 => (9, 1, 47),
        //             3 => (4, 4, 73),
        //             4 => (0, 7, 100),
        //             5 => (12, 44, 138),
        //             6 => (24, 82, 177),
        //             7 => (57, 125, 209),
        //             8 => (134, 181, 229),
        //             9 => (241, 233, 248),
        //             10 => (241, 233, 191),
        //             11 => (248, 201, 95),
        //             12 => (255, 170, 0),
        //             13 => (204, 128, 0),
        //             14 => (153, 87, 0),
        //             15 => (106, 52, 3),
        //             _ => unreachable!()
        //         };
        //         points_with_colours.push((point, rgb))
        //     }
        // }
        // points_with_colours

        let mut max_iterations_per_pixel: Vec<u64> = vec![0; self.max_iterations as usize];
        for (_, iteration) in &points {
            if *iteration < self.max_iterations {
                max_iterations_per_pixel[*iteration as usize] += 1
            }
        }
        let total_iterations_before_max: u64 = max_iterations_per_pixel.iter().sum();
        let mut hue;
        let mut rgb;
        for point in &points {
            hue = 0.0;
            if point.1 < self.max_iterations {
                for i in 0..point.1 {
                    hue += max_iterations_per_pixel[i as usize] as f64
                        / total_iterations_before_max as f64
                }
                //println!("{hue}");
                rgb = colorsys::Rgb::from(colorsys::Hsl::from((hue * 360.0, 100.0, 50.0)));
            } else {
                rgb = colorsys::Rgb::from((0, 0, 0))
            };
            points_with_colours.push((
                point.0,
                (
                    rgb.red().round() as u8,
                    rgb.green().round() as u8,
                    rgb.blue().round() as u8,
                ),
            ))
        }
        points_with_colours
    }

    pub fn points_with_colours_parallel(self) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        let points = self.points_with_iterations_parallel();
        let mut points_with_colours: Vec<(MandelbrotPoint, (u8, u8, u8))> = vec![];
        // for (point, iteration) in points {
        //     if iteration < self.max_iterations {
        //         let i = iteration % 16;
        //         let rgb = match i {
        //             0 => (66, 30, 15),
        //             1 => (25, 7, 26),
        //             2 => (9, 1, 47),
        //             3 => (4, 4, 73),
        //             4 => (0, 7, 100),
        //             5 => (12, 44, 138),
        //             6 => (24, 82, 177),
        //             7 => (57, 125, 209),
        //             8 => (134, 181, 229),
        //             9 => (241, 233, 248),
        //             10 => (241, 233, 191),
        //             11 => (248, 201, 95),
        //             12 => (255, 170, 0),
        //             13 => (204, 128, 0),
        //             14 => (153, 87, 0),
        //             15 => (106, 52, 3),
        //             _ => unreachable!()
        //         };
        //         points_with_colours.push((point, rgb))
        //     }
        // }
        // points_with_colours

        let mut max_iterations_per_pixel: Vec<u64> = vec![0; self.max_iterations as usize];
        for (_, iteration) in &points {
            if *iteration < self.max_iterations {
                max_iterations_per_pixel[*iteration as usize] += 1
            }
        }
        let total_iterations_before_max: u64 = max_iterations_per_pixel.iter().sum();
        points
            .into_par_iter()
            .map(|point| {
                let mut hue = 0.0;
                let mut rgb = Default::default();
                hue = 0.0;
                if point.1 < self.max_iterations {
                    for i in 0..point.1 {
                        hue += max_iterations_per_pixel[i as usize] as f64
                            / total_iterations_before_max as f64
                    }
                    //println!("{hue}");
                    rgb = colorsys::Rgb::from(colorsys::Hsl::from((hue * 360.0, 100.0, 50.0)));
                } else {
                    rgb = colorsys::Rgb::from((0, 0, 0))
                };
                (
                    point.0,
                    (
                        rgb.red().round() as u8,
                        rgb.green().round() as u8,
                        rgb.blue().round() as u8,
                    ),
                )
            })
            .collect()
    }
}
