use crate::{Stage, Style, Color}; 

/// Draws a line in cartesian coords from `xy1` to `xy2`.
///
/// Arguments: 
/// - stage: &mut [Stage] - stage to draw onto. 
/// - xy1: ([f32], [f32]) - coord for first point. 
/// - xy2: ([f32], [f32]) - coord for second point. 
/// - color: [Color] - struct containing rgba `[u8; 4]` color. 
pub fn line(
    stage: &mut Stage, 
    xy1: (f32, f32), 
    xy2: (f32, f32), 
    style: Style,
) { 
    let Some(xy1_px) = stage.world_to_pixel(xy1) else { return; }; 
    let Some(xy2_px) = stage.world_to_pixel(xy2) else { return; }; 
    line_px(stage, xy1_px, xy2_px, style.color); 
}


/// Draws a line in pixel coords from `xy1` to `xy2`.
pub(crate) fn line_px(
    stage: &mut Stage,
    xy1_px: (isize, isize), 
    xy2_px: (isize, isize),
    color: Color
) { 

    let (x1, y1) = xy1_px; 
    let (x2, y2) = xy2_px;

    // coords are guarded individually in 
    // Stage::word_to_pixel; roughly redundant guard 
    debug_assert!((x2 - x1) != isize::MIN);
    debug_assert!((y2 - y1) != isize::MIN);

    let mut x = x1; 
    let mut y = y1; 

    let dx = (x2 - x1).abs(); 
    let dy = (y2 - y1).abs(); 

    let sx = (x2 - x1).signum(); 
    let sy = (y2 - y1).signum();
 
    let rgba = color.rgba(); 

    // Bresenham line algorithm
    if dx >= dy { 
        let mut err = 2 * dy - dx; 

        for _ in 0..=dx { 
            stage.plot(x, y, rgba); 

            if err >= 0 { 
                y += sy; 
                err -= 2 * dx; 
            }

            x += sx; 
            err += 2 * dy; 

        }
    } else { 
        let mut err = 2 * dx - dy; 

        for _ in 0..=dy { 
            stage.plot(x, y, rgba); 

            if err >= 0 { 
                x += sx; 
                err -= 2 * dy; 
            }

            y += sy; 
            err += 2 * dx; 
        }
    }
}
