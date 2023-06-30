extern crate sdl2;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::{event::Event, pixels::Color, mouse::MouseButton};
use sdl2::rect::{Rect, Point};
use std::error::Error;
use std::ops::{Sub, Add, Div, Mul, AddAssign};

trait Widget {
    fn draw<T: RenderTarget>(&mut self, canvas: &mut Canvas::<T>) -> Result<(), String>
    where T: RenderTarget;
    fn update(&mut self, event: &Event);
}

struct Button {
    bound: Rect,
    on_color: Color,
    off_color: Color,
    state: bool,
}

impl Widget for Button {
    fn draw<T: RenderTarget>(&mut self, canvas: &mut Canvas::<T>) -> Result<(), String> {
        if self.state {
            canvas.set_draw_color(self.on_color);
        } else {
            canvas.set_draw_color(self.off_color);
        }
        canvas.fill_rect(self.bound)
    }
    fn update(&mut self, event: &Event) {
        if let Event::MouseButtonDown{mouse_btn, x, y, ..} = event {
            if *mouse_btn == MouseButton::Left && self.bound.contains_point(Point::new(*x, *y)) {
                self.state = !self.state;
            }
        }
    }
}

trait Numeric<T>: Copy + Sub<Output=T> + AddAssign
                    + Add<Output=T> + Mul<Output=T>
                    + Div<Output=T> + TryInto<f32> + From<f32>
                    + std::fmt::Display + std::cmp::PartialOrd{}
impl Numeric<f32> for f32 {}
struct Slider<T: Numeric<T>> {
    bound: Rect,
    fore: Color,
    back: Color,
    slider: Rect,
    value: T,
    start_slide_value: T,
    range: (T, T),
    start: i32,
    on_slide: bool
}
impl<T: Numeric<T>> Slider<T> {
    fn new(bound: Rect, fore: Color, back: Color, range: (T, T), slider_width: u32) -> Slider<T> {
        Slider {
            bound,
            fore,
            back,
            range,
            value: range.0,
            start_slide_value: range.0,
            on_slide: false,
            start: 0,
            slider: Rect::new(bound.x(),
                    bound.y(),
                    slider_width,
                    bound.height())
        }
    }
}
impl<T1: Numeric<T1>> Widget for Slider<T1> {
    fn draw<T2: RenderTarget>(&mut self, canvas: &mut Canvas::<T2>) -> Result<(), String> {
        canvas.set_draw_color(self.back);
        canvas.fill_rect(self.bound)?;
        canvas.set_draw_color(self.fore);
        canvas.fill_rect(self.slider)
    }
    fn update(&mut self, event: &Event) {
        match event {
            Event::MouseButtonDown{mouse_btn, x, y, ..} => {
                if  !self.on_slide && *mouse_btn == MouseButton::Left && self.slider.contains_point(Point::new(*x, *y)) {
                    self.start = *x;
                    self.start_slide_value = self.value;
                    self.on_slide = true;
                }
            },
            Event::MouseButtonUp{..} => {
                if self.on_slide { self.on_slide = false; }
            },
            Event::MouseMotion{x, ..} => {
                if self.on_slide {
                    let len = match (self.range.1 - self.range.0).try_into() {
                        Ok(x) => x,
                        Err(_) => { panic!("Can't parse T1 into f32"); }
                    };
                    self.value = self.start_slide_value + T1::from((*x - self.start) as f32 / (self.bound.width() - self.slider.width()) as f32 * len);
                    if self.value > self.range.1 {
                        self.value = self.range.1;
                    } else if self.value < self.range.0 {
                        self.value = self.range.0;
                    }
                    let percent =  match ((self.value - self.range.0) / T1::from(len)).try_into() {
                        Ok(x) => x,
                        Err(_) => { panic!("Can't parse T1 into f32"); }
                    };
                    self.slider.set_x(
                        self.bound.x() as i32
                        + (percent * (self.bound.width() - self.slider.width()) as f32) as i32);
                }
            },
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Some-ui-in-rust", 800, 600)
        // .resizable()
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut widget = Button {
        bound: Rect::new(100, 100, 300, 50),
        on_color: Color::BLUE,
        off_color: Color::GRAY,
        state: false,
    };
    let mut slider = Slider::new(Rect::new(200, 200, 300, 50), Color::BLUE, Color::RED, (0.0f32, 10.0f32), 50);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => {
                    widget.update(&event);
                    slider.update(&event);
                }
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        widget.draw(&mut canvas)?;
        slider.draw(&mut canvas)?;
        canvas.present();
    }

    Ok(())
}
