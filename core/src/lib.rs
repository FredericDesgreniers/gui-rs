#![feature(use_nested_groups)]

extern crate sdl2;
extern crate rayon;

mod rendering;
pub mod components;

use rendering::Renderer;
use components::component::Component;
use components::visuals::{VisualContext};
use rayon::prelude::*;

pub struct Application {
	renderer: Renderer,
	components: Vec<Box<Component>>
}

impl Application {
	pub fn new() -> Self {
		Application {
			renderer: Renderer::create().unwrap(),
			components: Vec::new()
		}
	}

	pub fn register_component(&mut self, component: Box<Component>) {
		self.components.push(component);
	}

	pub fn run(&mut self) {
		let window_size = self.renderer.window_size();

		let root_visual_context = VisualContext {
			position: (0, 0),
			dimension: (window_size.0, window_size.1),
		};

		self.components.par_iter_mut().for_each(|component| {
			component.update_visuals(&root_visual_context);
		});

		loop {
			if !self.renderer.process_events() {
				return;
			}

			self.components.par_iter_mut().for_each(|component| {
				component.tick();
			});

			let mut rendering_state = self.renderer.start_render();

			self.components.iter_mut().for_each(|component| {
				component.render(&mut rendering_state.with_offset(component.visual_context().position)).expect("Could not draw a component");
			});

			rendering_state.end();
		}
	}
}

