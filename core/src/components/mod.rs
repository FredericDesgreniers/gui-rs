pub mod positions;
pub mod visuals;
pub mod component;

use self::positions::*;
use self::visuals::*;
use self::component::Component;

use super::rendering::renderable::Renderable;
use super::rendering::RenderingState;


pub struct ComponentBase {
	pub position: ComponentPosition,
	pub dimension: ComponentDimension,

	pub visuals: VisualContext,

	children: Vec<Box<Component>>
}

impl ComponentBase {
	pub fn new(position: ComponentPosition, dimension: ComponentDimension) -> Self{
		ComponentBase {
			position,
			dimension,
			visuals: VisualContext::default(),
			children: Vec::new()
		}
	}

	pub fn register_child(&mut self, component: Box<Component>) {
		self.children.push(component);
	}
}

impl Component for ComponentBase {

}

impl Visual for ComponentBase {
	fn update_visuals(&mut self, parent: Option<VisualContext>) {
		self.visuals.position = match self.position {
			ComponentPosition::Fixed {x, y} => {
				(x,y)
			}
		};

		self.visuals.dimension = match self.dimension {
			ComponentDimension::Fixed {width, height} => {
				(width, height)
			},
			ComponentDimension::Percent {width, height} => {
				if let Some(parent_visual_context) = parent {
					((parent_visual_context.dimension.0 as f32 * width) as u32,
					(parent_visual_context.dimension.1 as f32 * height) as u32)
				} else {
					(0,0)
				}
			}
		};

		let visuals = self.visuals;
		self.children.iter_mut().for_each(|ref mut child| child.update_visuals(Some(visuals)));

	}
	fn visual_context(&self) -> &VisualContext {
		&self.visuals
	}
}

impl Renderable for ComponentBase {
	fn render(&self, rendering_state: &mut RenderingState) -> Result<(), String>{
		rendering_state.set_color(255, 0, 0);
		rendering_state.draw_rectangle((0, 0), self.visuals.dimension)?;

		for child in &self.children {
			child.render(&mut rendering_state.with_offset(child.visual_context().position))?;
		}

		Ok(())
	}
}