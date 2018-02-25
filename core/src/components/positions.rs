pub enum PositionValue {
	Fixed(i32),
	Percent(f32)
}

pub struct Position {
	pub x: PositionValue,
	pub y: PositionValue
}

pub enum DimensionValue {
	Fixed(u32),
	Percent(f32)
}

pub struct Dimension {
	pub width: DimensionValue,
	pub height: DimensionValue
}

