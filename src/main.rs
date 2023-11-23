use num::complex::ComplexFloat;
use num::Complex;

fn main() {
    let mandelbrot = MandelbrotPlane::new(0.25, 0.41, -0.06, 0.05, 1920, 1080, 1000);
    let points = mandelbrot.points_with_colours();
    let mut image = image::RgbImage::new(1920 ,1080);
    //println!("{points:?}");
    for point in points {
        image.put_pixel(
            ((point.0.point.re() - mandelbrot.re_min)
                / ((mandelbrot.re_max - mandelbrot.re_min) / (mandelbrot.width as f64)))
                .round() as u32,
            ((point.0.point.im() - mandelbrot.im_min)
                / ((mandelbrot.im_max - mandelbrot.im_min) / (mandelbrot.height as f64)))
                .round() as u32,
            image::Rgb(
            point.1.into()),

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
    pub fn points_with_colours(self) -> Vec<(MandelbrotPoint, (u8, u8, u8))> {
        let points = self.points_with_iterations();
        let mut points_with_colours: Vec<(MandelbrotPoint, (u8, u8, u8))> = vec![];
        let iterations = points.iter().map(|x| x.1);
        let total_iterations_before_max: u64 = iterations.filter(|x| *x!=self.max_iterations).sum();
        let mut max_iterations_per_pixel: Vec<u64> = vec![0; self.max_iterations as usize + 1];
        for (_, iterations) in &points {
            max_iterations_per_pixel[*iterations as usize] += 1
        }
        let mut hue;
        for point in &points {
            hue = 0.0;
            if point.1 != self.max_iterations {
            for i in 0..point.1 {
                hue += max_iterations_per_pixel[i as usize] as f64 / total_iterations_before_max as f64
            }
        }
            points_with_colours.push((point.0, (((hue*100.0%1.0)*255.0) as u8, ((hue*5.0%1.0)*255.0) as u8, ((hue*10.0%1.0)*255.0) as u8)))
        }
        points_with_colours
    }
}