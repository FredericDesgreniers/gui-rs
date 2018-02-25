#![feature(use_nested_groups)]

extern crate core;
use core::components::{ComponentBase, border::Border};
use core::components::positions::*;



pub fn main() {
	let mut application = core::Application::new();



	let mut component1 = Border::new((0,0,0),
									 Box::new(ComponentBase::new(
										 Position{x: PositionValue::Percent(0.1), y: PositionValue::Percent(0.1)},
										 Dimension {width: DimensionValue::Percent(0.8), height:DimensionValue::Percent(0.8)})
									 ));

	let component2 = Border::new((0, 0, 0),
								 Box::new(ComponentBase::new(
									 Position{x: PositionValue::Percent(0.5), y: PositionValue::Fixed(10)},
									 Dimension{width: DimensionValue::Fixed(300), height:DimensionValue::Fixed(50)})
								 ));



	component1.child().register_child(Box::new(component2));

	application.register_component(Box::new(component1));

	application.run();
}