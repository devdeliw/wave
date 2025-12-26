/// [`Color`] struct containing an RGBA `[u8; 4]` array.
#[derive(Debug, Clone, Copy)]
pub struct Color([u8; 4]);

impl Color {
    /// `Color([0, 0, 0, 0])`.
    pub const TRANSPARENT: Color = Color([0, 0, 0, 0]);
    /// Opaque Black. `Color([0, 0, 0, 255])`.
    pub const BLACK : Color = Color([0, 0, 0, 255]);
    /// Opaque White. `Color([255, 255, 255, 255])`.
    pub const WHITE : Color = Color([255, 255, 255, 255]);
    /// Opaque Red. `Color([255, 0, 0, 255])`.
    pub const RED   : Color = Color([255, 0, 0, 255]);
    /// Opaque Green. `Color([0, 255, 0, 255])`.
    pub const GREEN : Color = Color([0, 255, 0, 255]);
    /// Opaque Blue. `Color([0, 0, 255, 255])`.
    pub const BLUE  : Color = Color([0, 0, 255, 255]);

    /// Creates a [`Color`] from the provided RGBA array `[r, g, b, a]`.
    ///
    /// Arguments:
    /// color: [u8; 4] - RGBA array.
    pub fn new(color: [u8; 4]) -> Self {
        Self(color)
    }

    /// Returns stored RGBA array `[r, g, b, a]`.
    pub fn rgba(self) -> [u8; 4] {
        self.0
    }

    /// Replaces the intrinsic alpha of `self`.
    ///
    /// This sets the *intrinsic* alpha in the [`Color`]. If you're using
    /// [`Style`] / [`Fill`] / [`Stroke`], their [`Opacity`] additionally multiplies
    /// this intrinsic alpha to produce an effective alpha at draw time.
    ///
    /// Arguments:
    /// - opacity: [`Opacity`]
    pub fn with_alpha(self, opacity: Opacity) -> Self {
        let mut rgba = self.0;
        rgba[3] = opacity.as_u8();
        Self(rgba)
    }
}


/// [`Style`] struct containing visual options for a shape.
///
/// Fields:
/// - fill: Option<[`Fill`]> - if Some(fill), fills object interior with args from [`Fill`].
/// - stroke: Option<[`Stroke`]> - if Some(stroke), draws object stroke with args from [`Stroke`].
///
/// Opacity model:
/// - [`Color`] contains an *intrinsic* alpha channel (RGBA `a`).
/// - [`Fill`] / [`Stroke`] also contain an [`Opacity`] multiplier.
/// - The effective alpha used for rasterization is:
///   `effective_a = (color_a * opacity) / 255` (rounded).
#[derive(Clone, Copy)]
pub struct Style {
    /// If `Some(fill)`, fills the interior using [`Fill`] args.
    pub fill: Option<Fill>,
    /// If `Some(stroke)`, draws the boundary using [`Stroke`] args.
    pub stroke: Option<Stroke>,
}

impl Style {
    /// Creates a [`Style`] from optional fill and stroke colors.
    /// Other style args are not set by default and must be set using provided setters.
    ///
    /// Arguments:
    /// - fill: `Option<Color>` - if `Some(color)`, will fill, else no fill.
    /// - stroke: `Option<Color>` - if `Some(color)`, will stroke, else no stroke.
    ///
    /// If `fill`/`stroke` is `Some(color)`, the corresponding [`Fill`]/[`Stroke`] is created
    /// with [`Opacity::OPAQUE`] by default. [`Stroke`] width is set to 1 pixel by default. 
    pub const fn new(fill: Option<Color>, stroke: Option<Color>) -> Self {

        let f = match fill {
            Some(color) => Some(Fill::new(color, Opacity::OPAQUE)),
            None        => None,
        };

        let s = match stroke {
            Some(color) => Some(Stroke::new(color, Opacity::OPAQUE, 1.0)),
            None        => None,
        };
        Self { fill: f, stroke: s }
    }

    /// Creates a stroke-only [`Style`] with [`Opacity::OPAQUE`].
    ///
    /// Arguments:
    /// - stroke_color: [`Color`]
    pub const fn make_stroke(stroke_color: Color) -> Self {
        Self {
            fill: None,
            stroke: Some(Stroke::new(stroke_color, Opacity::OPAQUE, 1.0))
        }
    }

    /// Creates a fill-only [`Style`] with [`Opacity::OPAQUE`].
    ///
    /// Arguments:
    /// - fill_color: [`Color`]
    pub const fn make_fill(fill_color: Color) -> Self {
        Self {
            fill: Some(Fill::new(fill_color, Opacity::OPAQUE)),
            stroke: None
        }
    }

    /// Returns `true` if there exists a `Some(fill)` / `Some(stroke)`
    /// in either `stroke` or `fill` of `self`, otherwise returns `false`.
    pub fn fill_or_stroke_exists(&self) -> bool {
        self.fill.is_some() || self.stroke.is_some()
    }

    /// Sets the stroke of `self` to the given color.
    ///
    /// Arguments:
    /// - stroke_color: [`Color`]
    pub fn set_stroke(&mut self, stroke_color: Color) {
        self.stroke = Some(Stroke::new(stroke_color, Opacity::OPAQUE, 1.0));
    }

    /// Sets the fill of `self` to the given color.
    ///
    /// Arguments:
    /// - fill_color: [`Color`]
    pub fn set_fill(&mut self, fill_color: Color) {
        self.fill = Some(Fill::new(fill_color, Opacity::OPAQUE));
    }

    /// Sets the fill opacity of `self`. If `self.fill` is `None`, does nothing.
    ///
    /// Arguments:
    /// - fill_opacity: [`Opacity`]
    pub fn set_fill_opacity(&mut self, fill_opacity: Opacity) {
        if let Some(mut f) = self.fill {
            f.opacity = fill_opacity;
            self.fill = Some(f);
        }
    }

    /// Sets the stroke opacity of `self`. If `self.stroke` is `None`, does nothing.
    ///
    /// Arguments:
    /// - stroke_opacity: [`Opacity`]
    pub fn set_stroke_opacity(&mut self, stroke_opacity: Opacity) {
        if let Some(mut s) = self.stroke {
            s.opacity = stroke_opacity;
            self.stroke = Some(s);
        }
    }

    /// Sets the stroke width of `self`. If `self.stroke` is `None`, does nothing. 
    /// 
    /// Arguments: 
    /// - stroke_width: [f32] 
    pub fn set_stroke_width(&mut self, stroke_width: f32) { 
        if let Some(mut s) = self.stroke { 
            s.width = stroke_width; 
            self.stroke = Some(s);
        }
    }
}


/// Configures opacity for [`Style`] `fill/stroke_opacity` setters.
///
/// Multiplier for RGBA's intrinsic alpha.
#[derive(Clone, Copy)]
pub struct Opacity(u8);

impl Opacity {
    pub const TRANSPARENT: Self = Self(0);
    pub const OPAQUE: Self = Self(255);

    /// Constructs an [`Opacity`] from a float in [0.0, 1.0] (clamped).
    pub fn from_f32(x: f32) -> Self {
        let a = (x.clamp(0.0, 1.0) * 255.0).round() as u8;
        Self(a)
    }

    /// Returns the opacity [`u8`] stored in `self` in [0, 255].
    pub const fn as_u8(self) -> u8 { self.0 }
}


/// Configures fill options for a given shape.
///
/// Can be constructed with given [`Color`] and [`Opacity`] using `Fill::new(..)`.
#[derive(Clone, Copy)]
pub struct Fill {
    pub(crate) color: Color,
    pub(crate) opacity: Opacity,
}

/// Configures stroke options for a given shape.
///
/// Can be constructed with given [`Color`] and [`Opacity`] using `Stroke::new(..)`.
#[derive(Clone, Copy)]
pub struct Stroke {
    pub(crate) color: Color,
    pub(crate) opacity: Opacity,
    pub(crate) width: f32, 
}

impl Fill {
    /// Creates a [`Fill`] with the given color and opacity.
    ///
    /// Arguments:
    /// - color: [`Color`]: fill color.
    /// - opacity: [`Opacity`]: fill opacity.
    pub const fn new(color: Color, opacity: Opacity) -> Self {
        Self { color, opacity }
    }

    /// Returns the effective [`Color`] of a [`Fill`]. The opacity
    /// uses both the intrinsic RGB**A** from provided [`Color`] and the
    /// extrinsic [`Opacity`].
    #[inline(always)]
    pub fn rgba(self) -> Color {
        let mut rgba = self.color.rgba();
        let a = rgba[3] as u16;
        let f = self.opacity.as_u8() as u16;

        rgba[3] = ((a * f + 127) / 255) as u8;
        Color::new(rgba)
    }
}

impl Stroke {
    /// Creates a [`Stroke`] with the given color, opacity, and width.
    ///
    /// Arguments:
    /// - color: [`Color`]: fill color.
    /// - opacity: [`Opacity`]: fill opacity.
    /// - width: [f32]: stroke width
    pub const fn new(color: Color, opacity: Opacity, width: f32) -> Self {
        Self { color, opacity, width }
    }

    /// Returns the effective [`Color`] of a [`Stroke`]. The opacity
    /// uses both the intrinsic RGB**A** from provided [`Color`] and the
    /// extrinsic [`Opacity`].
    #[inline(always)]
    pub fn rgba(self) -> Color {
        let mut rgba = self.color.rgba();
        let a = rgba[3] as u16;
        let f = self.opacity.as_u8() as u16;

        rgba[3] = ((a * f + 127) / 255) as u8;
        Color::new(rgba)
    }
}

