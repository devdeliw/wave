use crate::Color;
use std::path::Path; 
use image::{ColorType, ImageFormat, ImageResult}; 


/// `Stage` struct containing a row major framebuffer
/// of length `width * height` containing RGBA `[u8; 4]`
/// array for each pixel.
pub struct Stage { 
    width: usize, 
    height: usize, 
    framebuf: Vec<[u8; 4]> 
}


/// Constructor and Getters.
impl Stage { 
    /// Creates a `width` x `height` [`Stage`] that is black and transparent.
    ///
    /// Arguments: 
    /// - width: [usize]: stage width. 
    /// - height: [usize]: stage height.
    ///
    /// Returns: 
    /// [`Stage`] of size `(width, height)`. 
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0, "Stage must be strictly positive in size"); 
        let length = width
            .checked_mul(height)
            .expect("Stage dimensions overflow");

        Self { 
            width, 
            height, 
            framebuf: vec![[0, 0, 0, 0]; length], 
        }
    }

    /// Returns the width of the [`Stage`].
    pub fn width(&self) -> usize { 
        self.width 
    }           

    /// Returns the height of the [`Stage`].
    pub fn height(&self) -> usize { 
        self.height
    }

    /// Returns the dimensions `(width, height)` of the [`Stage`].
    pub fn dimensions(&self) -> (usize, usize) { 
        (self.width, self.height)
    }

    /// Returns a reference to the [`Stage`] framebuffer.
    pub fn pixels(&self) -> &[[u8; 4]] { 
       &self.framebuf 
    }

    /// Returns a mutable reference to the [`Stage`] framebuffer.
    pub fn pixels_mut(&mut self) -> &mut [[u8; 4]] { 
        &mut self.framebuf
    }

    /// Gets the color value of a pixel at `(x, y)`.
    ///
    /// Returns `None` if out-of-bounds, otherwise `Some([u8; 4])`.
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<[u8; 4]> {

        // quick return
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = self.index(x, y);
        Some(self.framebuf[index])
    }
 
    /// Returns the number of pixels in the [`Stage`].
    pub fn len(&self) -> usize { 
        self.framebuf.len()
    }

    /// Returns `true` if Stage is empty.
    /// Effectively dead code, only here for clippy. 
    pub fn is_empty(&self) -> bool { 
        self.framebuf.is_empty()
    }
}


/// Setters. 
impl Stage { 
    /// Returns the linear framebuffer index for the pixel
    /// at `(x, y)` where `(0, 0)` is the top-left.
    fn index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        y * self.width + x
    }


    /// Sets the [`Stage`] background to the provided `color`. 
    pub fn clear(&mut self, color: Color) { 
        self.framebuf.fill(color.rgba()); 
    } 


    /// Sets the color value of a pixel at `(x, y)`.
    /// If the pixel is out-of-bounds, silently does nothing.
    #[inline(always)]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {

        // silent quick return 
        if x >= self.width || y >= self.height {
            return;
        }

        let color = color.rgba(); 
        let index = self.index(x, y);
        self.framebuf[index] = color;
    }


    /// Sets the color value of a signed pixel at `(x, y)`.
    /// If the pixel is out-of-bounds, silently does nothing.
    ///
    /// Hot path in drawing shapes.
    #[inline(always)]
    pub fn plot(&mut self, x: isize, y: isize, color: Color) {
        if x < 0 || y < 0 { 
            return; 
        } 

        let color = color.rgba(); 
        let (xu, yu) = (x as usize, y as usize);
        if xu < self.width && yu < self.height { 
            let idx = yu * self.width + xu;
            self.framebuf[idx] = color;
        }
    }
}

/// Helpers. 
impl Stage { 
    /// Returns the framebuffer as a contiguous `&[u8]` slice of RGBA bytes
    /// in row major order suitable for rendering.
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: 
        // framebuf MUST remain Vec<[u8; 4]>. arrays of u8 have no padding. 
        // so the data is tightly packed RGBA bytes.
        unsafe { 
            std::slice::from_raw_parts( 
                self.framebuf.as_ptr() as *const u8, 
                self.len() * std::mem::size_of::<[u8; 4]>()
            )
        }
    }

    /// Converts world/cartesian coordinates (origin at center) into
    /// pixel coordinates (origin top-left).
    ///
    /// Returns 
    /// - `Some(isize, isize)`: if pixel coordinate is finite and representable
    /// - `None`: otherwise
    pub(crate) fn world_to_pixel(&self, (x, y): (f32, f32)) -> Option<(isize, isize)> {
        if !x.is_finite() || !y.is_finite() { 
            return None; 
        } 

        let center_x = (self.width as f32 - 1.0) * 0.5; 
        let center_y = (self.height as f32 - 1.0) * 0.5; 

        let px = (x + center_x).round(); 
        let py = (center_y - y).round();

        if px < isize::MIN as f32 || px > isize::MAX as f32 { return None; }
        if py < isize::MIN as f32 || py > isize::MAX as f32 { return None; }

        Some((px as isize, py as isize))
    }

    /// Fills contiguous pixels at row `y` from `x0` to `x1` inclusive with `color`.
    /// `y`, `x0`, `x1` are in pixel coords. 
    pub(crate) fn fill_span(&mut self, y: isize, x0: isize, x1: isize, color: Color) {
        if y < 0 { return; } 
        let y = y as usize; 
        if y >= self.height { return; } 

        if x0 > x1 { return; }

        let mut a = x0; 
        let mut b = x1; 

        if b < 0 || a >= self.width as isize { return; } 
        a = a.max(0); 
        b = b.min(self.width as isize - 1);
        if a > b { return; }

        let row = y * self.width; 
        let color = color.rgba(); 
        self.framebuf[row + a as usize .. row + b as usize + 1].fill(color); 
    }


    /// Saves a [`Stage`] as a `png`. 
    pub fn save_png<P: AsRef<Path>>(&self, path: P) -> ImageResult<()> { 
        let (w, h) = self.dimensions(); 

        let bytes = self.as_bytes(); 
        assert_eq!(bytes.len(), w * h * 4); 

        image::save_buffer_with_format( 
            path, 
            bytes, 
            w as u32, 
            h as u32, 
            ColorType::Rgba8, 
            ImageFormat::Png, 
        )
    }
}

