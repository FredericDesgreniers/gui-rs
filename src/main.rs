extern crate core;
use core::components::{ComponentBase};
use core::components::positions::{ComponentPosition, ComponentDimension};


pub fn main() {
	let mut application = core::Application::new();

	let mut component1 = ComponentBase::new(ComponentPosition::Fixed{x: 10, y: 10}, ComponentDimension::Percent {width: 0.5, height:0.5});

	let component2 = ComponentBase::new(ComponentPosition::Fixed{x: 20, y: 10}, ComponentDimension::Fixed{width: 200, height:50});

	component1.register_child(Box::new(component2));

	application.register_component(Box::new(component1));

	application.run();
}