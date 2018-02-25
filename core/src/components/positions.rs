pub enum ComponentPosition {
	Fixed{x: i32, y: i32},

}


pub enum ComponentDimension {
	Fixed{width: u32, height:u32},
	Percent{width: f32, height: f32}
}


