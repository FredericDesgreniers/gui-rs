pub mod positions;
pub mod visuals;
pub mod component;
pub mod border;

use self::positions::*;
use self::visuals::*;
use self::component::Component;

use super::rendering::renderable::Renderable;
use super::rendering::RenderingState;


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
	fn update_visuals(&mut self, parent: Option<VisualContext>) {

		self.visuals.position = (
			match self.position.x {
				PositionValue::Fixed(x) => x,
				PositionValue::Percent(x_percent) => {
					if let Some(parent) = parent {
						(parent.dimension.0 as f32 * x_percent) as i32
					} else {
						0
					}
				}
			},
			match self.position.y {
				PositionValue::Fixed(y) => y,
				PositionValue::Percent(y_percent) => {
					if let Some(parent) = parent {
						(parent.dimension.1 as f32 * y_percent) as i32
					} else {
						0
					}
				}
			}
		);

		self.visuals.dimension = (
			match self.dimension.width {
				DimensionValue::Fixed(width) => width,
				DimensionValue::Percent(width_percent) => {
					if let Some(parent) = parent {
						(parent.dimension.0 as f32 * width_percent) as u32
					} else {
						0
					}
				}
			},
			match self.dimension.height {
				DimensionValue::Fixed(height) => height,
				DimensionValue::Percent(height_percent) => {
					if let Some(parent) = parent {
						(parent.dimension.1 as f32 * height_percent) as u32
					} else {
						0
					}
				}
			}
		);

		let visuals = self.visuals;
		self.children.iter_mut().for_each(|ref mut child| child.update_visuals(Some(visuals)));

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