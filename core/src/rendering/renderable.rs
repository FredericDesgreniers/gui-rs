use super::*;

pub trait Renderable {
	fn render(&self, rendering_state: &mut RenderingState) -> Result<(), String>;
}