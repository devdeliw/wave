use crate::{Stage, Style, Color}; 
use crate::primitives::line::draw_line_pxl; 

/// Returns sorted vertices by `y`-value.
fn sort_vertices(
    xy1: (isize, isize),
    xy2: (isize, isize),
    xy3: (isize, isize),
) -> [(isize, isize); 3] {
    let mut v = [xy1, xy2, xy3];
    v.sort_by_key(|p| p.1);
    v
}

/// Returns sorted span bounds at same `y`-value.
#[inline(always)]
fn sort_span_bounds(curx1: isize, curx2: isize) -> (isize, isize) {
    if curx1 <= curx2 { (curx1, curx2) } else { (curx2, curx1) }
}

#[inline(always)]
fn invslope_fp(dx: isize, dy: isize) -> i64 {
    ((dx as i64) << 16) / (dy as i64)
}

#[inline(always)]
fn fp_ceil_to_int(x_fp: i64) -> isize {
    ((x_fp + 0xFFFF) >> 16) as isize
}


/// Fills a flat-bottom triangle in pixel coords where `v1.y <= v2.y == v3.y`.
fn fill_flat_bottom_triangle(
    stage: &mut Stage,
    v1: (isize, isize),
    v2: (isize, isize),
    v3: (isize, isize),
    fill_color: Color,
) {
    let dy1 = v2.1 - v1.1;
    let dy2 = v3.1 - v1.1;
    if dy1 == 0 || dy2 == 0 { return; }
    let dxdy1 = invslope_fp(v2.0 - v1.0, dy1);
    let dxdy2 = invslope_fp(v3.0 - v1.0, dy2);

    let mut curx1: i64 = (v1.0 as i64) << 16;
    let mut curx2: i64 = (v1.0 as i64) << 16;

    // include top scanline, exclude bottom scanline.
    for y in v1.1..v2.1 {
        let xa = fp_ceil_to_int(curx1);
        let xb = fp_ceil_to_int(curx2);

        let (x0, mut x1) = sort_span_bounds(xa, xb);
        x1 -= 1;

        stage.fill_span_pxl(y, x0, x1, fill_color);

        curx1 += dxdy1;
        curx2 += dxdy2;
    }
}

/// Fills a flat-top triangle in pixel coords where `v1.y == v2.y <= v3.y`.
fn fill_flat_top_triangle(
    stage: &mut Stage,
    v1: (isize, isize),
    v2: (isize, isize),
    v3: (isize, isize),
    fill_color: Color,
) {
    let dy1 = v3.1 - v1.1;
    let dy2 = v3.1 - v2.1;
    if dy1 == 0 || dy2 == 0 { return; }

    let dxdy1 = invslope_fp(v3.0 - v1.0, dy1);
    let dxdy2 = invslope_fp(v3.0 - v2.0, dy2);
    let mut curx1: i64 = (v1.0 as i64) << 16;
    let mut curx2: i64 = (v2.0 as i64) << 16;

    // include top scanline, exclude bottom scanline.
    for y in v1.1..v3.1 {
        let xa = fp_ceil_to_int(curx1);
        let xb = fp_ceil_to_int(curx2);

        let (x0, mut x1) = sort_span_bounds(xa, xb);
        x1 -= 1;

        stage.fill_span_pxl(y, x0, x1, fill_color);

        curx1 += dxdy1;
        curx2 += dxdy2;
    }
}

/// Fills an arbitrary triangle in pixel coords.
fn fill_triangle(
    stage: &mut Stage,
    xy1: (isize, isize),
    xy2: (isize, isize),
    xy3: (isize, isize),
    fill_color: Color,
) {
    let [v1, v2, v3] = sort_vertices(xy1, xy2, xy3);
    let (x1, y1) = v1;
    let (_, y2) = v2;
    let (x3, y3) = v3;

    if y1 == y3 { return; }

    if y2 == y3 {
        fill_flat_bottom_triangle(stage, v1, v2, v3, fill_color);
    } else if y1 == y2 {
        fill_flat_top_triangle(stage, v1, v2, v3, fill_color);
    } else {
        let dy = y3 - y1;
        if dy == 0 { return; }

        let t_fp: i64 = (((y2 - y1) as i64) << 16) / (dy as i64);
        let x4: isize = x1 + (((t_fp * (x3 - x1) as i64) >> 16) as isize);
        let v4 = (x4, y2);

        fill_flat_bottom_triangle(stage, v1, v2, v4, fill_color);
        fill_flat_top_triangle(stage, v2, v4, v3, fill_color);
    }
}

/// Draws a triangle in pixel coords using three pixel coords. 
///
/// Arguments:
/// - stage: &mut [`Stage`] - stage to draw onto.
/// - xy1: ([isize], [isize]) - first coord.
/// - xy2: ([isize], [isize]) - second coord.
/// - xy3: ([isize], [isize]) - third coord.
/// - style: [`Style`] - struct containing styling args.
pub(crate) fn draw_triangle_pxl(
    stage: &mut Stage,
    xy1: (isize, isize),
    xy2: (isize, isize),
    xy3: (isize, isize),
    style: Style,
) {
    if !style.fill_or_stroke_exists() {
        return;
    }

    if let Some(fill) = style.fill {
        let fill_color = fill.rgba(); 
        fill_triangle(stage, xy1, xy2, xy3, fill_color);
    }

    if let Some(stroke) = style.stroke {
        let stroke_color = stroke.rgba(); 
        draw_line_pxl(stage, xy1, xy2, stroke_color);
        draw_line_pxl(stage, xy2, xy3, stroke_color);
        draw_line_pxl(stage, xy3, xy1, stroke_color);
    }
}

