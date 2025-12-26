//! Defines a general [Path] object for drawing shapes with a given [Style].
//!
//! Every primitive polygon object is built using a [Path].

use crate::{Color, Stage, Style};
use crate::primitives::triangle::draw_triangle; 

/// A general Path object.
///
/// Arguments:
/// - nodes: Vec<([f32], [f32])> - ordered collection of cartesian coords.
/// - closed: [bool] - whether to connect the last point with the first.
pub struct Path {
    nodes:  Vec<(f32, f32)>,
    closed: bool,
}

impl Path {
    /// Generates a [Path] from provided coordinates and closed [bool].
    ///
    /// Arguments:
    /// - nodes: Vec<([f32], [f32])> - ordered collection of cartesian coords.
    /// - closed: [bool] - whether to connect the last point with the first.
    pub fn new(nodes: Vec<(f32, f32)>, closed: bool) -> Self {
        Self { nodes, closed }
    }

    /// Converts `nodes` from cartesian `Vec<(f32, f32)>` to pixel `Option<Vec<(isize, isize)>>`.
    ///
    /// If any cartesian node is unrepresentable, bails and returns `None`.
    pub(crate) fn to_pixels(&self, stage: &mut Stage) -> Option<Vec<(isize, isize)>> {
        let mut out: Vec<(isize, isize)> = Vec::with_capacity(self.nodes.len());
        for &xy in &self.nodes {
            out.push(stage.world_to_pixel(xy)?);
        }
        Some(out)
    }

    /// Builds the line stroke of `self` with a given `width`. 
    pub(crate) fn make_stroke(
        nodes_px: &[(isize, isize)],
        closed: bool,
        width: f32,
        stage: &mut Stage,
        stroke_color: Color,
    ) {
        if nodes_px.len() < 2 {
            return;
        }
        if !width.is_finite() || width <= 0.0 {
            return;
        }

        let style = Style::make_fill(stroke_color);

        let mut i = 0;
        while i + 1 < nodes_px.len() {
            let xy1 = nodes_px[i];
            let xy2 = nodes_px[i + 1];

            if let Some([a, b, c, d]) = stroke_corners(xy1, xy2, width) {
                draw_triangle(stage, a, b, c, style);
                draw_triangle(stage, a, c, d, style);
            }

            i += 1;
        }

        if closed {
            let xy1 = nodes_px[nodes_px.len() - 1];
            let xy2 = nodes_px[0];

            if let Some([a, b, c, d]) = stroke_corners(xy1, xy2, width) {
                draw_triangle(stage, a, b, c, style);
                draw_triangle(stage, a, c, d, style);
            }
        }
    }

    /// Fills the interior of `self` in pixel coords.
    pub(crate) fn make_fill(
        nodes_px: &[(isize, isize)],
        stage: &mut Stage,
        fill_color: Color,
    ) {
        if nodes_px.len() < 3 {
            return;
        }

        let (ymin, ymax) = y_bound(nodes_px);
        if ymin >= ymax {
            return;
        }

        let h = stage.height() as isize;
        let y0 = ymin.max(0);
        let y1 = ymax.min(h - 1);
        if y0 > y1 {
            return;
        }

        let mut crossings: Vec<isize> = Vec::new();

        for y in y0..=y1 {
            crossings.clear();

            let mut i = 0;
            while i + 1 < nodes_px.len() {
                let (x1, y1e) = nodes_px[i];
                let (x2, y2e) = nodes_px[i + 1];

                if y1e != y2e {
                    let ylo = y1e.min(y2e);
                    let yhi = y1e.max(y2e);

                    if y >= ylo && y < yhi {
                        let x1f = x1 as f32;
                        let x2f = x2 as f32;
                        let y1f = y1e as f32;
                        let y2f = y2e as f32;

                        let x = x1f + (y as f32 - y1f) * (x2f - x1f) / (y2f - y1f);
                        crossings.push(x.floor() as isize);
                    }
                }

                i += 1;
            }

            let (x1, y1e) = nodes_px[nodes_px.len() - 1];
            let (x2, y2e) = nodes_px[0];

            if y1e != y2e {
                let ylo = y1e.min(y2e);
                let yhi = y1e.max(y2e);

                if y >= ylo && y < yhi {
                    let x1f = x1 as f32;
                    let x2f = x2 as f32;
                    let y1f = y1e as f32;
                    let y2f = y2e as f32;

                    let x = x1f + (y as f32 - y1f) * (x2f - x1f) / (y2f - y1f);
                    crossings.push(x.floor() as isize);
                }
            }

            crossings.sort_unstable();
            debug_assert!(crossings.len() % 2 == 0);

            let mut j = 0;
            while j + 1 < crossings.len() {
                let x1 = crossings[j];
                let x2 = crossings[j + 1];

                let l = x1 + 1;
                let r = x2 - 1;

                if l <= r {
                    stage.fill_span(y, l, r, fill_color);
                }

                j += 2;
            }
        }
    }

    /// Renders `self` on a `stage` using `style`.
    ///
    /// Arguments: 
    /// - stage: &mut [Stage] - stage to draw onto. 
    /// - style: [Style] - struct containing style args. 
    pub fn render(&self, stage: &mut Stage, style: Style) {
        let Some(nodes_px) = self.to_pixels(stage) else { return; };
        if !style.fill_or_stroke_exists() { return; };

        if self.closed {
            if let Some(fill) = style.fill {
                let fill_color = fill.rgba();
                Self::make_fill(&nodes_px, stage, fill_color);
            }
        }

        if let Some(stroke) = style.stroke {
            let stroke_color = stroke.rgba();
            Self::make_stroke(
                &nodes_px,
                self.closed,
                stroke.width,
                stage,
                stroke_color,
            );
        }
    }
}

fn y_bound(nodes_px: &[(isize, isize)]) -> (isize, isize) {
    let mut ymin = nodes_px[0].1;
    let mut ymax = nodes_px[0].1;

    for &(_, y) in &nodes_px[1..] {
        ymin = ymin.min(y);
        ymax = ymax.max(y);
    }

    (ymin, ymax)
}

/// Returns the corners of a line with a stroke `width`.
///
/// Order: `[p1+o, p2+o, p2-o, p1-o]`.
fn stroke_corners(
    xy1: (isize, isize),
    xy2: (isize, isize),
    width: f32,
) -> Option<[(isize, isize); 4]> {
    if !width.is_finite() || width <= 0.0 {
        return None;
    }

    let (x1, y1) = (xy1.0 as f32, xy1.1 as f32);
    let (x2, y2) = (xy2.0 as f32, xy2.1 as f32);

    let dx = x2 - x1;
    let dy = y2 - y1;

    let len2 = dx * dx + dy * dy;
    if len2 == 0.0 {
        return None;
    }

    let inv_len = len2.sqrt().recip();

    let nx = -dy * inv_len;
    let ny =  dx * inv_len;

    let r  = width * 0.5;
    let ox = nx * r;
    let oy = ny * r;

    let a = ((x1 + ox).round() as isize, (y1 + oy).round() as isize);
    let b = ((x2 + ox).round() as isize, (y2 + oy).round() as isize);
    let c = ((x2 - ox).round() as isize, (y2 - oy).round() as isize);
    let d = ((x1 - ox).round() as isize, (y1 - oy).round() as isize);

    Some([a, b, c, d])
}

