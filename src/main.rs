mod backend;

use backend::MandelbrotPlane;
use num::complex::ComplexFloat;
use num::Complex;

fn main() {
    let mandelbrot = MandelbrotPlane::new(-1.25, -1.2499, 0.0235, 0.0236, 1000, 1000, 10000);
    let points = mandelbrot.points_with_colours();
    let mut image = image::RgbImage::new(mandelbrot.width() as u32, mandelbrot.height() as u32);
    //println!("{points:?}");
    for point in points {
        image.put_pixel(
            ((point.0.point().re() - mandelbrot.re_min())
                / ((mandelbrot.re_max() - mandelbrot.re_min()) / (mandelbrot.width() as f64)))
                .round() as u32,
            ((point.0.point().im() - mandelbrot.im_min())
                / ((mandelbrot.im_max() - mandelbrot.im_min()) / (mandelbrot.height() as f64)))
                .round() as u32,
            image::Rgb(point.1.into()),
        );
    }

    image.save("output.png").unwrap();
    //println!("{:?}", image.pixels())
}
