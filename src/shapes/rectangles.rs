use crate::{Stage, Style}; 
use crate::shapes::lines::line_px; 


/// Draws a rectangle in cartesian coords *centered* about `origin` given `height` and `width`.
///
/// Arguments: 
/// - stage: &mut [Stage] - stage to draw onto. 
/// - origin: ([f32], [f32]) - coord for origin/center of rectangle. 
/// - height: [f32] - height of rectangle. 
/// - width: [f32] - width of rectangle. 
/// - style [Style] - struct containing styling args. 
pub fn rectangle(
    stage: &mut Stage,
    origin: (f32, f32),
    height: f32,
    width: f32,
    style: Style,
) {
    if !height.is_finite() || height <= 0.0 || !width.is_finite() || width <= 0.0 {
        return;
    }

    let Some((xc, yc)) = stage.world_to_pixel(origin) else { return; };

    let h = height.ceil().max(1.0) as isize;
    let w = width.ceil().max(1.0) as isize;

    let half_w = w / 2;
    let half_h = h / 2;

    // rectangle edges
    let l = xc - half_w;
    let r = l + (w - 1);

    let t0 = yc - half_h;      
    let b0 = t0 + (h - 1);     

    // clip y
    let y_min = 0;
    let y_max = stage.height() as isize - 1;
    let t = t0.max(y_min);
    let b = b0.min(y_max);

    if t > b { return; } 

    let rgba = style.color.rgba();

    // draw stroke
    line_px(stage, (l, t0), (r, t0), style.color);
    line_px(stage, (l, b0), (r, b0), style.color);
    line_px(stage, (l, t0), (l, b0), style.color);
    line_px(stage, (r, t0), (r, b0), style.color);

    // fill interior
    if style.fill {
        let x0 = l + 1;
        let x1 = r - 1;

        let y0 = (t0 + 1).max(y_min);
        let y1 = (b0 - 1).min(y_max);

        if x0 <= x1 && y0 <= y1 {
            for y in y0..=y1 {
                stage.fill_span(y, x0, x1, rgba); 
            }
        }
    }

}

/// Draws a square in cartesian coords from given `side_length` centered about `origin`.
///
/// Arguments:
/// - stage: &mut [Stage] - stage to draw onto. 
/// - origin: ([f32], [f32]) - coord for origin/center of rectangle. 
/// - side_length: [f32] - side length of square.
/// - style [Style] - struct containing styling args.
pub fn square(
    stage: &mut Stage, 
    origin: (f32, f32), 
    side_length: f32, 
    style: Style,
) { 
    rectangle(stage, origin, side_length, side_length, style); 
}
