use super::positions::*;
use super::visuals::*;
use super::component::Component;
use super::super::rendering::renderable::Renderable;
use super::super::rendering::RenderingState;
use rayon::prelude::*;

pub struct ComponentBase {
	pub position: Position,
	pub dimension: Dimension,

	pub visuals: VisualContext,

	children: Vec<Box<Component>>
}

impl ComponentBase {
	pub fn new(position: Position, dimension: Dimension) -> Self{
		ComponentBase {
			position,
			dimension,
			visuals: VisualContext::default(),
			children: Vec::new()
		}
	}
}

impl Component for ComponentBase {
	fn register_child(&mut self, component: Box<Component>) {
		self.children.push(component);
	}
}

impl Visual for ComponentBase {
	fn update_visuals(&mut self, parent: &VisualContext) {
		self.visuals.update_using_parent(&parent, Some(self.position), Some(self.dimension));

		let visuals = &self.visuals;
		self.children.par_iter_mut().for_each(|ref mut child| child.update_visuals(visuals));

	}
	fn visual_context(&self) -> &VisualContext {
		&self.visuals
	}
}

impl Renderable for ComponentBase {
	fn render(&self, rendering_state: &mut RenderingState) -> Result<(), String>{
		rendering_state.set_color(255, 255, 255);
		rendering_state.fill_rect((0, 0), self.visuals.dimension)?;

		for child in &self.children {
			child.render(&mut rendering_state.with_offset(child.visual_context().position))?;
		}

		Ok(())
	}
}
