use crate::{Stage, Color}; 
use crate::shapes::lines::line_px; 

const SQRT_3: f32 = 1.7320508;

/// Draws a triangle in cartesian coords using three coords. For equilateral triangles, use
/// [equilateral_triangle].
///
/// Arguments: 
/// - stage: &mut [Stage] - stage to draw onto. 
/// - xy1: ([f32], [f32]) - first coord.
/// - xy2: ([f32], [f32]) - second coord.
/// - xy3: ([f32], [f32]) - third coord.
/// - color [Color] - struct containing rgba `[u8; 4]` color.
pub fn triangle(
    stage: &mut Stage, 
    xy1: (f32, f32), 
    xy2: (f32, f32), 
    xy3: (f32, f32), 
    color: Color 
) { 
    let Some(xy1_px) = stage.world_to_pixel(xy1) else { return; }; 
    let Some(xy2_px) = stage.world_to_pixel(xy2) else { return; }; 
    let Some(xy3_px) = stage.world_to_pixel(xy3) else { return; }; 

    line_px(stage, xy1_px, xy2_px, color); 
    line_px(stage, xy2_px, xy3_px, color); 
    line_px(stage, xy3_px, xy1_px, color); 
}


/// Draws an equilateral triangle in cartesian coords centered about `origin` of given
/// `side_length`. For arbitrary triangles, use [triangle].
///
/// Arguments: 
/// - stage: &mut [Stage] - stage to draw onto. 
/// - origin: ([f32], [f32]) - center coord of equilateral triangle. 
/// - side_length: side length of equilateral triangle. 
/// - color [Color] - struct containing rgba `[u8; 4]` color.
pub fn equilateral_triangle(
    stage: &mut Stage,
    origin: (f32, f32),
    side_length: f32,
    color: Color,
) {
    if !side_length.is_finite() || side_length <= 0.0 {
        return;
    }

    let (xc, yc) = origin;
    let apex_dy = (SQRT_3 / 3.0) * side_length;
    let base_dy = (SQRT_3 / 6.0) * side_length;

    let xy1 = (xc, yc + apex_dy);
    let xy2 = (xc - side_length * 0.5, yc - base_dy);
    let xy3 = (xc + side_length * 0.5, yc - base_dy);

    triangle(stage, xy1, xy2, xy3, color);
}

