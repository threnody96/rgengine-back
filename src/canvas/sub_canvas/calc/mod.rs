use super::super::VirtualCanvas;
use ::sdl2::rect::{ Point, Rect };

impl VirtualCanvas {

    pub fn calc_bounding_rect(rect: Rect, angle: f64) -> Rect {
        let points = Self::calc_after_rotated_points(rect, angle);
        Self::calc_bounding_rect_by_points(points)
    }

    fn calc_after_rotated_points(rect: Rect, angle: f64) -> Vec<Point> {
        let rad = angle.to_radians();
        let center = rect.center();
        vec![
            Self::calc_bounding_point(rect.left(), rect.top(), center, rad),
            Self::calc_bounding_point(rect.left(), rect.bottom(), center, rad),
            Self::calc_bounding_point(rect.right(), rect.top(), center, rad),
            Self::calc_bounding_point(rect.right(), rect.bottom(), center, rad)
        ]
    }

    fn calc_bounding_rect_by_points(mut points: Vec<Point>) -> Rect {
        let first_p = points.pop().unwrap();
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (first_p.x(), first_p.y(), first_p.x(), first_p.y());
        for p in &points {
            if min_x > p.x() { min_x = p.x(); }
            if min_y > p.y() { min_y = p.y(); }
            if max_x < p.x() { max_x = p.x(); }
            if max_y < p.y() { max_y = p.y(); }
        }
        Rect::new(min_x, min_y, (max_x - min_x + 1) as u32, (max_y - min_y + 1) as u32)
    }

    fn calc_bounding_point(base_x: i32, base_y: i32, center: Point, rad: f64) -> Point {
        let (sin, cos) = (rad.sin(), rad.cos());
        let (x, y) = ((base_x - center.x()) as f64, (base_y - center.y()) as f64);
        Point::new(
            ((x * cos - y * sin).round() as i32) + center.x(),
            ((x * sin + y * cos).round() as i32) + center.y()
        )
    }

}
