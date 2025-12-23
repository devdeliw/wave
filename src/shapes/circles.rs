use crate::{Stage, Style}; 

/// Draws a circle in cartesian coords centered at `origin` with given `radius`. 
///
/// Arguments: 
/// - stage: &mut [Stage] - stage to draw onto. 
/// - origin: ([f32], [f32]) - coord for origin of circle. 
/// - radius: [f32] - radius of circle. 
/// - style: [Style] - struct containing styling args. 
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

    let fill_rgba = style.fill.map(|c| c.rgba()); 
    let stroke_rgba = style.stroke.map(|c| c.rgba()); 

    let (xc, yc) = origin_px;
    let r = radius_px as isize;

    let mut x = 0;
    let mut y = r;
    let mut d = 3 - 2 * r;

    loop {
        // fill 
        if let Some(rgba) = fill_rgba {
            stage.fill_span(yc + y, xc - x + 1, xc + x - 1, rgba);
            stage.fill_span(yc - y, xc - x + 1, xc + x - 1, rgba);
            stage.fill_span(yc + x, xc - y + 1, xc + y - 1, rgba);
            stage.fill_span(yc - x, xc - y + 1, xc + y - 1, rgba);
        }

        // stroke
        if let Some(rgba) = stroke_rgba {
            stage.plot(xc + x, yc + y, rgba);
            stage.plot(xc - x, yc + y, rgba);
            stage.plot(xc + x, yc - y, rgba);
            stage.plot(xc - x, yc - y, rgba);
            stage.plot(xc + y, yc + x, rgba);
            stage.plot(xc - y, yc + x, rgba);
            stage.plot(xc + y, yc - x, rgba);
            stage.plot(xc - y, yc - x, rgba);
        }

        if x >= y { break; }

        // Bresenham circle
        x += 1;
        if d > 0 {
            y -= 1;
            d += 4 * (x - y) + 10;
        } else {
            d += 4 * x + 6;
        }
    }
}

