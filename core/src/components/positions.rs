#[derive(Copy, Clone)]
pub enum PositionValue {
	Fixed(i32),
	Percent(f32)
}

#[derive(Copy, Clone)]
pub struct Position {
	pub x: PositionValue,
	pub y: PositionValue
}

#[derive(Copy, Clone)]
pub enum DimensionValue {
	Fixed(u32),
	Percent(f32)
}

#[derive(Copy, Clone)]
pub struct Dimension {
	pub width: DimensionValue,
	pub height: DimensionValue
}

