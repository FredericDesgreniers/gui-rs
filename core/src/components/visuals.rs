use super::Renderable;
use super::positions::*;

pub trait Visual : Renderable{
	fn update_visuals(&mut self, parent: &VisualContext);
	fn visual_context(&self) -> &VisualContext;
}

#[derive(Default, Copy, Clone)]
pub struct VisualContext {
	pub position: (i32, i32),
	pub dimension: (u32, u32)
}

impl VisualContext {
	pub fn update_using_parent(&mut self, parent: &VisualContext, position: Option<Position>, dimension: Option<Dimension>)
	{
		if let Some(position) = position {
			self.update_position_using_parent(parent, position);
		}

		if let Some(dimension) = dimension {
			self.update_dimension_using_parent(parent, dimension);
		}
	}

	pub fn update_position_using_parent(&mut self, parent: &VisualContext, position: Position) {
		 self.position = (
			match position.x {
				PositionValue::Fixed(x) => x,
				PositionValue::Percent(x_percent) => {
					(parent.dimension.0 as f32 * x_percent) as i32
				}
			},
			match position.y {
				PositionValue::Fixed(y) => y,
				PositionValue::Percent(y_percent) => {
					(parent.dimension.1 as f32 * y_percent) as i32
				}
			}
		);
	}

	pub fn update_dimension_using_parent(&mut self, parent: &VisualContext, dimension: Dimension) {
		self.dimension = (
			match dimension.width {
				DimensionValue::Fixed(width) => width,
				DimensionValue::Percent(width_percent) => {
					(parent.dimension.0 as f32 * width_percent) as u32
				}
			},
			match dimension.height {
				DimensionValue::Fixed(height) => height,
				DimensionValue::Percent(height_percent) => {
					(parent.dimension.1 as f32 * height_percent) as u32
				}
			}
		);
	}
}