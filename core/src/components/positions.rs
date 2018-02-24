use std::ops::{Add, Sub};

pub enum ComponentPosition {
	Fixed{x: i32, y: i32}
}

impl Add<ComponentDimension> for ComponentPosition {
	type Output = (i32, i32);

	fn add(self, dimension: ComponentDimension) -> Self::Output {

		let position: (i32, i32) = match self {
			ComponentPosition::Fixed{x , y} => {
				(x,y)
			}
		};

		let dimension: (u32, u32) = match dimension {
			ComponentDimension::Fixed{width, height} => {
				(width, height)
			}
		};

		(position.0 + dimension.0 as i32, position.1 + dimension.1 as i32)
	}
}

impl Sub<ComponentDimension> for ComponentPosition {
	type Output = (i32, i32);

	fn sub(self, dimension: ComponentDimension) -> Self::Output {

		let position: (i32, i32) = match self {
			ComponentPosition::Fixed{x , y} => {
				(x,y)
			}
		};

		let dimension: (u32, u32) = match dimension {
			ComponentDimension::Fixed{width, height} => {
				(width, height)
			}
		};

		(position.0 - dimension.0 as i32, position.1 - dimension.1 as i32)
	}
}

pub enum ComponentDimension {
	Fixed{width: u32, height:u32}
}


