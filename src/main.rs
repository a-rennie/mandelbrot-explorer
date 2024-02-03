#![allow(dead_code)]
mod backend;
mod colours;
mod renderer;

use crate::colours::*;
use crate::renderer::mandelbrot_from_params_parallel;
use iced::event::Status;
use iced::mouse::Cursor;
use iced::widget::canvas::Event;
use iced::widget::{button, canvas, column, pick_list, row, slider, text};
use iced::{Element, Length, Point, Rectangle, Sandbox, Settings, Size};
use num::complex::ComplexFloat;
use num::Complex;
use std::fmt::Formatter;

const CANVAS_SIZE: u16 = 500; // square canvas

fn main() -> iced::Result {
    MandelbrotExplorer::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ZoomIn(Point),
    ZoomOut(Point),
    IterationSet(u32),
    Refresh,
    RenderImage,
    ColourSelected(Colour),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Colour {
    Default,
    Rainbow,
}
impl Colour {
    fn to_array(self) -> &'static [(u8, u8, u8)] {
        match self {
            Colour::Default => &DEFAULT_COLOURS,
            Colour::Rainbow => &RAINBOW_COLOURS,
        }
    }
}
impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Colour::Default => "Default",
                Colour::Rainbow => "Rainbow",
            }
        )
    }
}
struct MandelbrotExplorer {
    set: MandelbrotSet,
}

impl Sandbox for MandelbrotExplorer {
    type Message = Message;

    fn new() -> Self {
        Self {
            set: MandelbrotSet::new(CANVAS_SIZE),
        }
    }

    fn title(&self) -> String {
        "Mandelbrot Set Explorer".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        // stops floating point inaccuracies being visible in image, but don't know how much performance impact this has
        self.set.resolution = self.set.resolution.clamp(2_f64.powi(-53), f64::MAX);
        match message {
            Message::ZoomIn(point) => {
                self.set.centre += Complex::new(
                    (point.x as f64 - (CANVAS_SIZE as f64 / 2.0)) * self.set.resolution,
                    (point.y as f64 - (CANVAS_SIZE as f64 / 2.0)) * self.set.resolution,
                );
                self.set.resolution *= 0.5;
                self.set.cache.clear()
            }
            Message::ZoomOut(point) => {
                self.set.centre += Complex::new(
                    (point.x as f64 - (CANVAS_SIZE as f64 / 2.0)) * self.set.resolution,
                    (point.y as f64 - (CANVAS_SIZE as f64 / 2.0)) * self.set.resolution,
                );
                self.set.resolution *= 2.0;
                self.set.cache.clear()
            }
            Message::IterationSet(num) => self.set.max_iterations = num as u64,
            Message::Refresh => self.set.cache.clear(),
            Message::RenderImage => {
                let centre = self.set.centre;
                let resolution = self.set.resolution;
                let max_iterations = self.set.max_iterations;
                let colour = self.set.colour.unwrap_or(Colour::Default).to_array();
                std::thread::spawn(move || {
                    let points = mandelbrot_from_params_parallel(
                        centre,
                        resolution / 8.0,
                        max_iterations,
                        4000,
                        4000,
                        colour,
                    );
                    let mut image = image::RgbImage::new(4000, 4000);
                    for point in points {
                        image.put_pixel(
                            point.0 .0 as u32,
                            point.0 .1 as u32,
                            image::Rgb(point.1.into()),
                        )
                    }
                    let _ = image.save("output.png");
                });
            }
            Message::ColourSelected(colour) => {
                self.set.colour = Some(colour);
                self.set.cache.clear()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            canvas::Canvas::new(&self.set)
                .width(CANVAS_SIZE)
                .height(CANVAS_SIZE),
            row![
                text(format!("Iterations: {:?}", self.set.max_iterations as u32)),
                slider(
                    0..=10000,
                    self.set.max_iterations as u32,
                    Message::IterationSet
                )
                .width(Length::Fill)
            ]
            .padding(10)
            .spacing(20),
            text(format!(
                "Centre: {} + {}i, Zoom: {}",
                self.set.centre.re(),
                self.set.centre.im(),
                1.0 / self.set.resolution
            )),
            row![
                button(text("Refresh Image")).on_press(Message::Refresh),
                button(text("Render 4000x4000 image")).on_press(Message::RenderImage),
                pick_list(
                    &[Colour::Default, Colour::Rainbow][..],
                    self.set.colour,
                    Message::ColourSelected
                )
            ]
        ]
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
    colour: Option<Colour>,
    cache: canvas::Cache,
}

impl MandelbrotSet {
    fn new(size: u16) -> MandelbrotSet {
        MandelbrotSet {
            max_iterations: 1000,
            centre: Complex::new(0.0, 0.0),
            resolution: 4.0 / size as f64,
            colour: Some(Colour::Default),
            ..Default::default()
        }
    }
}

impl canvas::Program<Message> for MandelbrotSet {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                        Some(Message::ZoomIn(cursor_position))
                    }
                    iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right) => {
                        Some(Message::ZoomOut(cursor_position))
                    }
                    iced::mouse::Event::WheelScrolled { delta } => match delta {
                        iced::mouse::ScrollDelta::Lines { x: _, y } => {
                            if y > 0.0 {
                                Some(Message::ZoomIn(cursor_position))
                            } else if y < 0.0 {
                                Some(Message::ZoomOut(cursor_position))
                            } else {
                                None
                            } // don't react to horizontal scrolling (yet)
                        }
                        iced::mouse::ScrollDelta::Pixels { x: _, y } => {
                            if y > 0.0 {
                                Some(Message::ZoomIn(cursor_position))
                            } else if y < 0.0 {
                                Some(Message::ZoomOut(cursor_position))
                            } else {
                                None
                            } // don't react to horizontal scrolling (yet)
                        }
                    },
                    _ => None,
                };
                (Status::Captured, message)
            }
            _ => (Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry> {
        let geom = self.cache.draw(renderer, bounds.size(), |frame| {
            frame.stroke(
                &canvas::Path::rectangle(Point::ORIGIN, frame.size()),
                canvas::Stroke::default(),
            );
            let points = mandelbrot_from_params_parallel(
                self.centre,
                self.resolution,
                self.max_iterations,
                frame.width().round() as u64,
                frame.height().round() as u64,
                &self.colour.unwrap_or(Colour::Default).to_array(),
            );
            for point in points {
                let path = canvas::Path::rectangle(
                    Point::new(point.0 .0 as f32, point.0 .1 as f32),
                    Size::new(1.0, 1.0),
                );
                frame.stroke(
                    &path,
                    canvas::Stroke {
                        style: canvas::Style::Solid(iced::Color {
                            r: point.1 .0 as f32 / 255.0,
                            g: point.1 .1 as f32 / 255.0,
                            b: point.1 .2 as f32 / 255.0,
                            a: 1.0,
                        }),
                        ..Default::default()
                    },
                )
            }
        });
        vec![geom]
    }
}
