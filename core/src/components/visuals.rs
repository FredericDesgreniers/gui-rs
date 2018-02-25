use super::Renderable;

pub trait Visual : Renderable{
	fn update_visuals(&mut self, parent: Option<VisualContext>);
}

#[derive(Default, Copy, Clone)]
pub struct VisualContext {
	pub position: (i32, i32),
	pub dimension: (u32, u32)
}