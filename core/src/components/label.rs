use super::component::Component;
use super::visuals::{Visual, VisualContext};
use super::super::rendering::{RenderingState, renderable::Renderable};
use super::positions::*;


struct Label {
	position: Position,
	visuals: VisualContext,
}

impl Label {
	pub fn new(position: Position, text: &str) -> Self {
		unimplemented!()
	}
}

impl Component for Label {
	fn register_child(&mut self, _: Box<Component>) {
		unimplemented!()
	}
}

impl Visual for Label {
	fn update_visuals(&mut self, parent: &VisualContext) {


		self.visuals.update_using_parent(&parent, Some(self.position), Some(self.dimension));
	}

	fn visual_context(&self) -> &VisualContext {
		&self.visuals
	}
}

impl Renderable for Label {
	fn render(&self, rendering_state: &mut RenderingState) -> Result<(), String> {
		unimplemented!()
	}
}