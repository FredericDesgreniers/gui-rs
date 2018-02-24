pub mod positions;
pub mod visuals;

use self::positions::*;
use self::visuals::*;

use super::rendering::renderable::Renderable;
use super::rendering::RenderingState;


pub struct Component {
	pub position: ComponentPosition,
	pub dimension: ComponentDimension,

	pub visual_position: (i32, i32),
	pub visual_dimension: (u32, u32)
}

impl Component {
	pub fn new(position: ComponentPosition, dimension: ComponentDimension) -> Self{
		Component {
			position,
			dimension,
			visual_position: (0,0),
			visual_dimension: (0,0)
		}
	}
}

impl Visual for Component {
	fn update_visuals(&mut self) {
		self.visual_position = match self.position {
			ComponentPosition::Fixed {x, y} => {
				(x,y)
			}
		};

		self.visual_dimension = match self.dimension {
			ComponentDimension::Fixed {width, height} => {
				(width, height)
			}
		}
	}
}

impl Renderable for Component {
	fn render(&self, rendering_state: &mut RenderingState) -> Result<(), String>{
		rendering_state.set_color(255, 0, 0);
		rendering_state.draw_rectangle(self.visual_position, self.visual_dimension)
	}
}