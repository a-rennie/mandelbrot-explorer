mod backend;
mod renderer;

use iced::mouse::Cursor;
use iced::widget::{canvas, column};
use iced::{Color, Element, Length, Point, Rectangle, Sandbox, Settings, Size};
use iced::event::Status;
use iced::widget::canvas::{Event, Program};
use num::complex::ComplexFloat;
use num::Complex;
use crate::renderer::mandelbrot_from_params_parallel;

fn main() -> iced::Result {
    // // let mandelbrot = MandelbrotPlane::new(-1.25, -1.2499, 0.0235, 0.0236, 1000, 1000, 10000);
    // // let points = renderer::mandelbrot_xy_coordinates_with_colours(mandelbrot);
    // // let mut image = image::RgbImage::new(mandelbrot.width() as u32, mandelbrot.height() as u32);
    // // for point in points {
    // //     image.put_pixel(
    // //         point.0 .0 as u32,
    // //         point.0 .1 as u32,
    // //         image::Rgb(point.1.into()),
    // //     )
    // // }
    //
    // let points = renderer::mandelbrot_from_params_parallel(
    //     Complex::new(-0.863527217, 0.238368848),
    //     0.001 / 4000.0,
    //     10000,
    //     1000,
    //     1000,
    // );
    // let mut image = image::RgbImage::new(1000, 1000);
    // //println!("{points:?}");
    // for point in points {
    //     image.put_pixel(
    //         point.0 .0 as u32,
    //         point.0 .1 as u32,
    //         image::Rgb(point.1.into()),
    //     )
    // }
    // // let points = mandelbrot.points_with_colours();
    // // let mut image = image::RgbImage::new(mandelbrot.width() as u32, mandelbrot.height() as u32);
    // // //println!("{points:?}");
    // // for point in points {
    // //     image.put_pixel(
    // //         ((point.0.point().re() - mandelbrot.re_min())
    // //             / ((mandelbrot.re_max() - mandelbrot.re_min()) / (mandelbrot.width() as f64)))
    // //             .round() as u32,
    // //         ((point.0.point().im() - mandelbrot.im_min())
    // //             / ((mandelbrot.im_max() - mandelbrot.im_min()) / (mandelbrot.height() as f64)))
    // //             .round() as u32,
    // //         image::Rgb(point.1.into()),
    // //     );
    // // }
    //
    // image.save("output.png").unwrap();
    // //println!("{:?}", image.pixels())
    MandelbrotExplorer::run(Settings::default())
}

#[derive(Debug)]
enum Message {
    PointClicked(Point),
}
struct MandelbrotExplorer {
    set: MandelbrotSet,
}

impl Sandbox for MandelbrotExplorer {
    type Message = Message;

    fn new() -> Self {
        Self {
            set: MandelbrotSet::new()
        }
    }

    fn title(&self) -> String {
        "Mandelbrot Explorer".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PointClicked(point) => {self.set.centre += Complex::new((point.x as f64 - 250.0) * self.set.resolution, (point.y as f64 - 250.0) * self.set.resolution); self.set.resolution *= 0.1;  self.set.cache.clear()}
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![canvas::Canvas::new(&self.set).width(500).height(500)]
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .into()
    }
}

#[derive(Default, Debug)]
struct MandelbrotSet {
    max_iterations: u64,
    centre: Complex<f64>,
    resolution: f64,
    cache: canvas::Cache,
}

impl canvas::Program<Message> for MandelbrotSet {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        let geom = self.cache.draw(renderer, bounds.size(), |frame| {
            frame.stroke(
                &canvas::Path::rectangle(Point::ORIGIN, frame.size()),
                canvas::Stroke::default(),
            );
            let points = mandelbrot_from_params_parallel(self.centre, self.resolution, self.max_iterations, frame.width().round() as u64, frame.height().round() as u64);
            for point in points {
                let path = canvas::Path::rectangle(Point::new(point.0.0 as f32, point.0.1 as f32), Size::new(1.0, 1.0));
                frame.stroke(&path, canvas::Stroke {
                    style: canvas::Style::Solid(iced::Color{r: point.1.0 as f32 / 255.0, g: point.1.1 as f32 / 255.0, b: point.1.2 as f32 / 255.0, a: 1.0}),
                    ..Default::default()
                })
            }
        });
        vec![geom]
    }

    fn update(&self, _state: &mut Self::State, event: Event, bounds: Rectangle, cursor: Cursor) -> (Status, Option<Message>) {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    iced::mouse::Event::ButtonPressed(
                        iced::mouse::Button::Left,
                    ) => Some(Message::PointClicked(cursor_position)),
                    _ => None,
                };
                (Status::Captured, message)
            }
            _ => (Status::Ignored, None),
        }
    }
}

impl MandelbrotSet {
    fn new() -> MandelbrotSet {
        MandelbrotSet {
            max_iterations: 1000,
            centre: Complex::new(0.0, 0.0),
            resolution: 4.0/500.0,
            ..Default::default()
        }
    }
}