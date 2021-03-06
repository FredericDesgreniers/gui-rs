pub mod renderable;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub struct Renderer {
	context: sdl2::Sdl,
	canvas: Canvas<Window>
}

impl Renderer {
	pub fn create() -> Result<Self, String> {
		let context = sdl2::init()?;
		let video_subsystem = context.video()?;
		if let Ok(window) = video_subsystem.window("Demo GUI app", 800, 600)
						 .position_centered()
						 .build() {
			let mut canvas = window.into_canvas().build().unwrap();

			Ok(Renderer {
				context,
				canvas
			})
		}else {
			Err(String::from("Could not create subsystem... "))
		}
	}

	pub fn window_size(&self) -> (u32, u32){
		(800, 600)
	}

	pub fn process_events(&mut self) -> bool{
		let mut event_pump = self.context.event_pump().unwrap();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					return false;
				},
				_ => {}
			}
		}
		return true;
	}

	pub fn start_render(&mut self) -> RenderingState{
		RenderingState::new(&mut self.canvas)
	}
}

pub struct RenderingState<'a> {
	pub canvas: &'a mut Canvas<sdl2::video::Window>,
	offset: (i32, i32),
	is_root_state: bool
}

impl<'a> RenderingState<'a> {
	fn new(canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>) -> Self {
		canvas.set_draw_color(Color::RGB(200, 200, 200));
		canvas.clear();

		RenderingState {
			canvas,
			offset: (0,0),
			is_root_state: true
		}
	}

	pub fn with_offset(&mut self, offset: (i32, i32)) -> RenderingState {
		RenderingState {
			canvas: self.canvas,
			offset: (self.offset.0 + offset.0, self.offset.1 + offset.1),
			is_root_state: false
		}
	}

	pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
		self.canvas.set_draw_color(Color::RGB(r, g, b));
	}

	pub fn draw_rectangle(&mut self, position: (i32, i32), dimensions: (u32, u32)) -> Result<(), String> {
		self.canvas.draw_rect(Rect::new(position.0 + self.offset.0, position.1 + self.offset.1, dimensions.0, dimensions.1))
	}

	pub fn fill_rect(&mut self, position: (i32, i32), dimensions: (u32, u32)) -> Result<(), String> {
		self.canvas.fill_rect(Rect::new(position.0 + self.offset.0, position.1 + self.offset.1, dimensions.0, dimensions.1))
	}

	pub fn end(self) {

	}
}

impl<'a> Drop for RenderingState<'a> {
	fn drop(&mut self) {
		if !self.is_root_state {
			return;
		}
		self.canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}