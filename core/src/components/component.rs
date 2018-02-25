
use super::visuals::Visual;
use rendering::renderable::Renderable;


pub trait Component: Visual + Renderable {
	fn register_child(&mut self, Box<Component>);
}