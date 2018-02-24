pub mod positions;
pub mod visuals;

use self::positions::*;
use self::visuals::*;

use super::rendering::renderable::Renderable;
use super::rendering::RenderingState;


pub struct Component {
	pub position: ComponentPosition,
	pub dimension: ComponentDimension,

	visuals: VisualContext
}

impl Component {
	pub fn new(position: ComponentPosition, dimension: ComponentDimension) -> Self{
		Component {
			position,
			dimension,
			visuals: VisualContext::default()
		}
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
		}
	}
}

impl Renderable for Component {
	fn render(&self, rendering_state: &mut RenderingState) -> Result<(), String>{
		rendering_state.set_color(255, 0, 0);
		rendering_state.draw_rectangle(self.visuals.position, self.visuals.dimension)
	}
}