pub mod positions;
pub mod visuals;

use self::positions::*;
use self::visuals::*;

use super::rendering::renderable::Renderable;
use super::rendering::RenderingState;


pub struct Component {
	pub position: ComponentPosition,
	pub dimension: ComponentDimension,

	pub visuals: VisualContext,

	children: Vec<Component>
}

impl Component {
	pub fn new(position: ComponentPosition, dimension: ComponentDimension) -> Self{
		Component {
			position,
			dimension,
			visuals: VisualContext::default(),
			children: Vec::new()
		}
	}

	pub fn register_child(&mut self, component: Component) {
		self.children.push(component);
	}
}

impl Visual for Component {
	fn update_visuals(&mut self, parent: Option<VisualContext>) {
		self.visuals.position = match self.position {
			ComponentPosition::Fixed {x, y} => {
				(x,y)
			}
		};

		self.visuals.dimension = match self.dimension {
			ComponentDimension::Fixed {width, height} => {
				(width, height)
			}
		};

		let visuals = self.visuals;
		self.children.iter_mut().for_each(|ref mut child| child.update_visuals(Some(visuals)));

	}
}

impl Renderable for Component {
	fn render(&self, rendering_state: &mut RenderingState) -> Result<(), String>{
		rendering_state.set_color(255, 0, 0);
		rendering_state.draw_rectangle((0, 0), self.visuals.dimension)?;

		for child in &self.children {
			child.render(&mut rendering_state.with_offset(child.visuals.position))?;
		}

		Ok(())
	}
}