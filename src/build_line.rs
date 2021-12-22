#![allow(clippy::approx_constant)]
use lyon::geom::{vector, point};
use lyon::path::builder::SvgPathBuilder;

pub fn build_line<Builder: SvgPathBuilder>(path: &mut Builder) {
    path.move_to(point(122.0, 69.0));
    path.relative_vertical_line_to(1.0);
    path.close();
}