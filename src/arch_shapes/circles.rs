use crate::{Stage, Style}; 

/// Draws a circle in cartesian coords centered at `origin` with given `radius`. 
///
/// Arguments: 
/// - stage: &mut [`Stage`] - stage to draw onto. 
/// - origin: ([f32], [f32]) - coord for origin of circle. 
/// - radius: [f32] - radius of circle. 
/// - style: [`Style`] - struct containing styling args. 
pub fn circle( 
    stage: &mut Stage, 
    origin: (f32, f32), 
    radius: f32, 
    style: Style,
) {
    // silent guard 
    if !radius.is_finite() || radius <= 0.0 { 
        return; 
    }

    let Some(origin_px) = stage.world_to_pixel(origin) else { return; }; 
    let radius_px = radius.ceil().max(1.0) as usize; 
    circle_px(stage, origin_px, radius_px, style);
}

/// Draws a circle in pixel-coordinate space with `radius_px` and `origin_px`.
pub(crate) fn circle_px(
    stage: &mut Stage,
    origin_px: (isize, isize),
    radius_px: usize,
    style: Style,
) {
    if !style.fill_or_stroke_exists() {
        return;
    }

    let fill_rgba   = style.fill.map(|f| f.rgba());
    let stroke_rgba = style.stroke.map(|s| s.rgba());

    let (xc, yc) = origin_px;
    let r = radius_px as isize;
    if r <= 0 {
        return;
    }

    let r_i64 = r as i64;
    let r2: i64 = r_i64 * r_i64;

    let rin: isize = (r - 1).max(0);
    let rin_i64 = rin as i64;
    let rin2: i64 = rin_i64 * rin_i64;

    let mut x_out: isize = r;
    let mut x_out2: i64  = r2;

    let mut x_in: isize  = rin;
    let mut x_in2: i64   = rin2;

    let shrink_x = |
        mut x: isize, 
        mut x2: i64, 
        y2: i64, 
        limit2: i64
    | -> (isize, i64) {
        while x > 0 && x2 + y2 > limit2 {
            x2 -= 2 * (x as i64) - 1;
            x -= 1;
        }

        (x, x2)
    };

    let mut y2: i64 = 0;

    for y in 0..=r {
        (x_out, x_out2) = shrink_x(x_out, x_out2, y2, r2);

        let xin_row: isize = if rin == 0 {
            -1
        } else {
            (x_in, x_in2) = shrink_x(x_in, x_in2, y2, rin2);
            if x_in2 + y2 <= rin2 { x_in } else { -1 }
        };

        let y_top = yc - y;
        let y_bot = yc + y;

        // strict interior fill 
        if let (Some(c), true) = (fill_rgba, xin_row >= 0) {
            stage.fill_span(y_top, xc - xin_row, xc + xin_row, c);
            if y != 0 {
                stage.fill_span(y_bot, xc - xin_row, xc + xin_row, c);
            }
        }

        // stroke 
        if let Some(c) = stroke_rgba {
            let a = xin_row + 1; 
            if a <= x_out {
                if a <= 0 {
                    stage.fill_span(y_top, xc - x_out, xc + x_out, c);

                    if y != 0 {
                        stage.fill_span(y_bot, xc - x_out, xc + x_out, c);
                    }

                } else {
                    stage.fill_span(y_top, xc - x_out, xc - a, c);
                    stage.fill_span(y_top, xc + a,   xc + x_out, c);

                    if y != 0 {
                        stage.fill_span(y_bot, xc - x_out, xc - a, c);
                        stage.fill_span(y_bot, xc + a,   xc + x_out, c);
                    }
                }
            }
        }

        y2 += 2 * (y as i64) + 1;
    }
}

