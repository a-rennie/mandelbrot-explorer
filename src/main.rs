use num::complex::ComplexFloat;
use num::Complex;

fn main() {
    let mandelbrot = MandelbrotPlane::new(-2.0, 1.5, -1.2, 1.2, 700, 480, 255);
    let points = mandelbrot.points_with_iterations();
    let mut image = image::RgbImage::new(700 ,480);
    //println!("{points:?}");
    for point in points {
        image.put_pixel(
            ((point.0.point.re() - mandelbrot.re_min)
                / ((mandelbrot.re_max - mandelbrot.re_min) / (mandelbrot.width as f64)))
                .round() as u32,
            ((point.0.point.im() - mandelbrot.im_min)
                / ((mandelbrot.im_max - mandelbrot.im_min) / (mandelbrot.height as f64)))
                .round() as u32,
            image::Rgb([
                255 - point.1 as u8,
                255 - point.1 as u8,
                255 - point.1 as u8,
            ]),

        );

       // println!("{},{}",mandelbrot.im_min, mandelbrot.im_max);
       //  println!("{},{}",((point.0.point.re() - mandelbrot.re_min)
       //      / ((mandelbrot.re_max - mandelbrot.re_min) / (mandelbrot.width as f64)))
       //      .round() as u32,
       //           ((point.0.point.im() - mandelbrot.im_min)
       //               / ((mandelbrot.im_max - mandelbrot.im_min) / (mandelbrot.height as f64)))
       //               .round() as u32);
    }

    image.save("output.png").unwrap();
    //println!("{:?}", image.pixels())
}
#[derive(Debug, Copy, Clone)]
struct MandelbrotPoint {
    point: Complex<f64>,
    max_iterations: u64,
}

impl MandelbrotPoint {
    pub fn new(point: Complex<f64>, max_iterations: u64) -> MandelbrotPoint {
        MandelbrotPoint {
            point,
            max_iterations,
        }
    }

    pub fn iterations(self) -> u64 {
        let mut iteration = 0;
        let mut z = Complex::new(0.0, 0.0);
        while z.norm_sqr() <= 4.0 && iteration < self.max_iterations {
            // finding square of distance much faster than sqrt()ing
            z = z * z + self.point;
            iteration += 1
        }
        iteration
    }
}

#[derive(Debug, Copy, Clone)]
struct MandelbrotPlane {
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

    pub fn points_with_iterations(self) -> Vec<(MandelbrotPoint, u64)> {
        let mut points = Vec::new();
        let mut point;
        for real in 0..self.width {
            for imaginary in 0..self.height {
                point = MandelbrotPoint::new(
                    Complex::new(
                        (((self.re_max - self.re_min) / (self.width as f64)) * real as f64)
                            + self.re_min,
                        (((self.im_max - self.im_min) / (self.height as f64)) * imaginary as f64) + self.im_min),
                    self.max_iterations,
                );
                points.push((point, point.iterations()))
            }
        }
        points
    }
}
