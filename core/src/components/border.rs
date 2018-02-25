use super::component::Component;
use super::visuals::{Visual, VisualContext};
use super::super::rendering::{RenderingState, renderable::Renderable};

pub struct Border {
	rgb: (u8, u8, u8),
	visuals: VisualContext,
	child: Box<Component>
}

impl Border {
	pub fn new(rgb: (u8, u8, u8), child: Box<Component>) -> Self {
		Border {
			rgb,
			visuals: VisualContext::default(),
			child
		}
	}
	pub fn child(&mut self) -> &mut Box<Component> {
		&mut self.child
	}
}

impl Component for Border {
	fn register_child(&mut self, component: Box<Component>) {
		self.child = component;
	}
}

impl Visual for Border {
	fn update_visuals(&mut self, parent: &VisualContext) {
		self.child.update_visuals(parent);
		self.visuals = self.child.visual_context().clone();
	}

	fn visual_context(&self) -> &VisualContext {
		&self.visuals
	}
}

impl Renderable for Border {
	fn render(&self, rendering_state: &mut RenderingState) -> Result<(), String> {
		self.child.render(rendering_state)?;
		rendering_state.set_color(self.rgb.0, self.rgb.1, self.rgb.2);
		rendering_state.draw_rectangle((0, 0), self.visuals.dimension)
	}
}