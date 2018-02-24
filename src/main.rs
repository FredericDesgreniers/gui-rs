extern crate core;
use core::components::{Component};
use core::components::positions::{ComponentPosition, ComponentDimension};


pub fn main() {
	let mut application = core::Application::new();

	application.register_component(Component::new(ComponentPosition::Fixed{x: 0, y: 0}, ComponentDimension::Fixed{width: 300, height:100}));

	application.run();
}