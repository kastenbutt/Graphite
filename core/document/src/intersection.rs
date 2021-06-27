use glam::{DAffine2, DMat3, DVec2};
use kurbo::{BezPath, Line, PathSeg, Point, Shape, Vec2};

pub fn intersect_quad_bez_path(quad: [Point; 4], shape: &BezPath) -> bool {
	let lines = vec![Line::new(quad[0], quad[1]), Line::new(quad[1], quad[2]), Line::new(quad[2], quad[3]), Line::new(quad[3], quad[0])];
	// check if outlines intersect
	for path_segment in shape.segments() {
		for line in &lines {
			if !path_segment.intersect_line(*line).is_empty() {
				return true;
			}
		}
	}
	// check if selection is entirely within the shape
	if shape.contains(quad[0]) {
		return true;
	}
	// check if shape is entirely within the selection
	if let Some(shape_point) = get_arbitrary_point_on_path(shape) {
		let mut pos = 0;
		let mut neg = 0;
		for line in lines {
			if line.p0 == shape_point {
				return true;
			};
			let line_vec = Vec2::new(line.p1.x - line.p0.x, line.p1.y - line.p0.y);
			let point_vec = Vec2::new(line.p1.x - shape_point.x, line.p1.y - shape_point.y);
			let cross = line_vec.cross(point_vec);
			if cross > 0.0 {
				pos += 1;
			} else if cross < 0.0 {
				neg += 1;
			}
			if pos > 0 && neg > 0 {
				return false;
			}
		}
	}
	true
}

pub fn get_arbitrary_point_on_path(path: &BezPath) -> Option<Point> {
	path.segments().next().map(|seg| match seg {
		PathSeg::Line(line) => line.p0,
		PathSeg::Quad(quad) => quad.p0,
		PathSeg::Cubic(cubic) => cubic.p0,
	})
}

pub fn point_line_segment_dist(x: Point, a: Point, b: Point) -> f64 {
	if (a - b).dot(x - b) * (b - a).dot(x - a) >= 0.0 {
		let mat = DMat3::from_cols_array(&[a.x, a.y, 1.0, b.x, b.y, 1.0, x.x, x.y, 1.0]);
		(mat.determinant() / (b - a).hypot()).abs()
	} else {
		f64::sqrt(f64::min((a - x).hypot2(), (b - x).hypot2()))
	}
}

pub fn transform_kurbo_point(point: &Point, transform: &DAffine2) -> Point {
	let original = DVec2::new(point.x, point.y);
	let transformed = transform.transform_point2(original);
	Point::new(transformed.x, transformed.y)
}
