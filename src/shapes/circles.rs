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


#[inline(always)]
fn fill_row(stage: &mut Stage, xc: isize, yc: isize, x: isize, y: isize, rgba: [u8; 4]) { 
    stage.fill_span(yc + y, xc - x, xc + x, rgba);
    stage.fill_span(yc - y, xc - x, xc + x, rgba); 
    stage.fill_span(yc + x, xc - y, xc + y, rgba); 
    stage.fill_span(yc - x, xc - y, xc + y, rgba); 
}

/// Circles are symmetric about `xc` and `yc`. After calculating the `(x, y)` 
/// pixel to color, we also know 7 other pixel coordinates to color. 
#[inline(always)] 
fn draw_octants(
    stage: &mut Stage, 
    xc: isize, 
    yc: isize, 
    x: isize, 
    y: isize, 
    rgba: [u8; 4], 
    fill: bool, 
) { 
    stage.plot(xc + x, yc + y, rgba); 
    stage.plot(xc - x, yc + y, rgba); 
    stage.plot(xc + x, yc - y, rgba); 
    stage.plot(xc - x, yc - y, rgba); 
    stage.plot(xc + y, yc + x, rgba); 
    stage.plot(xc - y, yc + x, rgba); 
    stage.plot(xc + y, yc - x, rgba); 
    stage.plot(xc - y, yc - x, rgba);   

    if fill { 
        fill_row(stage, xc, yc, x, y, rgba); 
    }
}

/// Draws a circle in pixel-coordinate space with `radius` and `origin`.
pub(crate) fn circle_px(
    stage: &mut Stage, 
    origin_px: (isize, isize), 
    radius_px: usize, 
    style: Style 
) {
    let color = style.color; 
    let fill = style.fill;
    let (xc, yc) = origin_px; 
    let rgba = color.rgba(); 
    let r = radius_px as isize;
    let (mut x, mut y) = (0, r);
    let mut d = 3 - 2 * r; 

    // Bresenham midpoint circle algorithm
    draw_octants(stage, xc, yc, x, y, rgba, fill); 
    while x < y { 
        x += 1; 

        if d > 0 { 
            y -= 1; 
            d += 4 * (x - y) + 10; 
        } else { 
            d += 4 * x + 6; 
        }

        draw_octants(stage, xc, yc, x, y, rgba, fill);
    }
} 
