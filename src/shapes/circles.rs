use crate::{Stage, Style};

/// Draws a circle in world coords centered at `origin` with given `radius`.
///
/// Arguments:
/// - stage: &mut [`Stage`] - stage to draw onto.
/// - origin: ([f32], [f32]) - world coord for circle center.
/// - radius: [f32] - radius in world units.
/// - style: [`Style`] - struct containing styling args.
pub fn circle(
    stage: &mut Stage,
    origin: (f32, f32),
    radius: f32,
    style: Style,
) {
    if !radius.is_finite() || radius <= 0.0 {
        return;
    }

    let Some(origin_pxl) = stage.world_to_pxl(origin) else { return; };

    let r0_pxl = radius.ceil().max(1.0) as isize;
    circle_pxl(stage, origin_pxl, r0_pxl, style);
}

/// Draws a circle in pixel-coordinate space with nominal radius `r0_pxl`.
fn circle_pxl(
    stage: &mut Stage,
    origin_pxl: (isize, isize),
    r0_pxl: isize,
    style: Style,
) {
    if !style.fill_or_stroke_exists() {
        return;
    }
    if r0_pxl <= 0 {
        return;
    }

    let fill_rgba = style.fill.map(|f| f.rgba());
    let stroke = style.stroke;
    let stroke_rgba = stroke.map(|s| s.rgba());

    let (r_out, r_in) = if let Some(s) = stroke {
        let w = s.width;
        if !w.is_finite() || w <= 0.0 {
            (r0_pxl, r0_pxl)
        } else {
            let half_out = (0.5 * w).ceil() as isize;
            let half_in  = (0.5 * w).floor() as isize;
            let r_out = r0_pxl + half_out;
            let r_in  = (r0_pxl - half_in).max(0);
            (r_out, r_in)
        }
    } else {
        (r0_pxl, r0_pxl)
    };

    let r_fill = if fill_rgba.is_some() {
        if stroke_rgba.is_some() { r_in } else { r0_pxl }
    } else {
        0
    };

    let (xc, yc) = origin_pxl;

    let r_out_i64 = r_out as i64;
    let r_out2: i64 = r_out_i64 * r_out_i64;

    let r_in_i64 = r_in as i64;
    let r_in2: i64 = r_in_i64 * r_in_i64;

    let r_fill_i64 = r_fill as i64;
    let r_fill2: i64 = r_fill_i64 * r_fill_i64;

    let mut x_out: isize = r_out;
    let mut x_out2: i64 = r_out2;

    let mut x_in: isize = r_in;
    let mut x_in2: i64 = r_in2;

    let mut x_fill: isize = r_fill;
    let mut x_fill2: i64 = r_fill2;

    let shrink_x = |mut x: isize, mut x2: i64, y2: i64, limit2: i64| -> (isize, i64) {
        while x > 0 && x2 + y2 > limit2 {
            x2 -= 2 * (x as i64) - 1;
            x -= 1;
        }
        (x, x2)
    };

    let mut y2: i64 = 0;

    for y in 0..=r_out {
        (x_out, x_out2) = shrink_x(x_out, x_out2, y2, r_out2);

        let x_in_row: isize = if stroke_rgba.is_some() {
            if r_in == 0 {
                -1
            } else {
                (x_in, x_in2) = shrink_x(x_in, x_in2, y2, r_in2);
                if x_in2 + y2 <= r_in2 { x_in } else { -1 }
            }
        } else {
            -1
        };

        let x_fill_row: isize = if let Some(_) = fill_rgba {
            if r_fill == 0 {
                -1
            } else {
                (x_fill, x_fill2) = shrink_x(x_fill, x_fill2, y2, r_fill2);
                if x_fill2 + y2 <= r_fill2 { x_fill } else { -1 }
            }
        } else {
            -1
        };

        let y_top = yc - y;
        let y_bot = yc + y;

        if let (Some(c), true) = (fill_rgba, x_fill_row >= 0) {
            stage.fill_span_pxl(y_top, xc - x_fill_row, xc + x_fill_row, c);
            if y != 0 {
                stage.fill_span_pxl(y_bot, xc - x_fill_row, xc + x_fill_row, c);
            }
        }

        if let Some(c) = stroke_rgba {
            let a = x_in_row + 1;
            if a <= x_out {
                if a <= 0 {
                    stage.fill_span_pxl(y_top, xc - x_out, xc + x_out, c);
                    if y != 0 {
                        stage.fill_span_pxl(y_bot, xc - x_out, xc + x_out, c);
                    }
                } else {
                    stage.fill_span_pxl(y_top, xc - x_out, xc - a, c);
                    stage.fill_span_pxl(y_top, xc + a, xc + x_out, c);
                    if y != 0 {
                        stage.fill_span_pxl(y_bot, xc - x_out, xc - a, c);
                        stage.fill_span_pxl(y_bot, xc + a, xc + x_out, c);
                    }
                }
            }
        }

        y2 += 2 * (y as i64) + 1;
    }
}

