use std::cmp::{max, min};

use super::{has_verticies::HasVerticies, verticies::Verticies};

pub fn get_bounding_verticies<TContainer>(container: &Vec<TContainer>) -> Verticies
where
	TContainer: HasVerticies,
{
	if !container.is_empty() {
		let mut left_x = None;
		let mut top_y = None;
		let mut right_x = None;
		let mut bottom_y = None;

		for vert in container
			.iter()
			.flat_map(|container| container.verticies().iter())
		{
			left_x = Some(match left_x {
				Some(old_left_x) => min(old_left_x, vert[0]),
				None => vert[0],
			});

			top_y = Some(match top_y {
				Some(old_top_y) => min(old_top_y, vert[1]),
				None => vert[1],
			});

			right_x = Some(match right_x {
				Some(old_right_y) => max(old_right_y, vert[0]),
				None => vert[0],
			});

			bottom_y = Some(match bottom_y {
				Some(old_bottom_y) => max(old_bottom_y, vert[1]),
				None => vert[1],
			});
		}

		[
			[
				left_x.expect("Not a single vertex was found in paragraph?"),
				top_y.unwrap(),
			],
			[right_x.unwrap(), top_y.unwrap()],
			[right_x.unwrap(), bottom_y.unwrap()],
			[left_x.unwrap(), bottom_y.unwrap()],
		]
	} else {
		Verticies::default()
	}
}
