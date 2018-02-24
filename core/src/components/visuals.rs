use super::Renderable;

pub trait Visual : Renderable{
	fn update_visuals(&mut self);
}