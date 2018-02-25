extern crate sdl2;

mod rendering;
pub mod components;

use rendering::Renderer;
use components::Component;
use components::visuals::Visual;

use rendering::renderable::*;



pub struct Application {
	renderer: Renderer,
	components: Vec<Component>
}

impl Application {
	pub fn new() -> Self {
		Application {
			renderer: Renderer::create().unwrap(),
			components: Vec::new()
		}
	}

	pub fn register_component(&mut self, component: Component) {
		self.components.push(component);
	}

	pub fn run(&mut self) {
		self.components.iter_mut().for_each(|ref mut component| {
			component.update_visuals(None);
		});

		loop {
			if !self.renderer.process_events() {
				return;
			}

			let mut rendering_state = self.renderer.start_render();

			self.components.iter_mut().for_each(|ref mut component| {
				component.render(&mut rendering_state.with_offset(component.visuals.position)).expect("Could not draw a component");
			});

			rendering_state.end();

		}

	}
}

