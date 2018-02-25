
use super::visuals::Visual;
use rendering::renderable::Renderable;

pub trait Component: Visual + Renderable + Send{
	fn register_child(&mut self, Box<Component>);
	fn tick(&mut self) {}
}