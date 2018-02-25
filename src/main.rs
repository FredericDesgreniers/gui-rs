extern crate core;
use core::components::{Component};
use core::components::positions::{ComponentPosition, ComponentDimension};


pub fn main() {
	let mut application = core::Application::new();

	let mut component1 = Component::new(ComponentPosition::Fixed{x: 60, y: 90}, ComponentDimension::Fixed{width: 300, height:100});

	let component2 = Component::new(ComponentPosition::Fixed{x: 20, y: 10}, ComponentDimension::Fixed{width: 200, height:50});

	component1.register_child(component2);

	application.register_component(component1);

	application.run();
}