#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use raqote::{
    BlendMode, DrawOptions, DrawTarget, LineCap, LineJoin, ExtendMode, FilterMode,
    Path, PathBuilder, Point, SolidSource, Source, StrokeStyle, Transform, Winding,
    Color, Gradient, GradientStop, Image, Spread,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_point {
    x: f32,
    y: f32,
}

impl From<rq_point> for Point {
    fn from(value: rq_point) -> Self {
        Point::new(value.x, value.y)
    }
}

impl From<Point> for rq_point {
    fn from(value: Point) -> Self {
        Self { x: value.x, y: value.y }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_transform {
    m11: f32,
    m12: f32,
    m21: f32,
    m22: f32,
    m31: f32,
    m32: f32,
}

impl From<rq_transform> for Transform {
    fn from(value: rq_transform) -> Self {
        Transform::new(value.m11, value.m12, value.m21, value.m22, value.m31, value.m32)
    }
}

impl From<Transform> for rq_transform {
    fn from(value: Transform) -> Self {
        Self {
            m11: value.m11,
            m12: value.m12,
            m21: value.m21,
            m22: value.m22,
            m31: value.m31,
            m32: value.m32,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct rq_color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_gradient_stop {
    position: f32,
    color: rq_color,
}


impl From<rq_color> for SolidSource {
    fn from(value: rq_color) -> Self {
        SolidSource::from_unpremultiplied_argb(value.a, value.r, value.g, value.b)
    }
}

impl From<rq_color> for Color {
    fn from(value: rq_color) -> Self {
        Color::new(value.a, value.r, value.g, value.b)
    }
}

impl From<rq_gradient_stop> for GradientStop {
    fn from(value: rq_gradient_stop) -> Self {
        GradientStop {
            position: value.position,
            color: value.color.into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rq_fill_rule {
    Winding,
    EvenOdd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rq_spread_mode {
    Pad,
    Reflect,
    Repeat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rq_extend_mode {
    Pad,
    Repeat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rq_filter_mode {
    Nearest,
    Bilinear,
}

impl From<rq_fill_rule> for Winding {
    fn from(value: rq_fill_rule) -> Self {
        match value {
            rq_fill_rule::Winding => Winding::NonZero,
            rq_fill_rule::EvenOdd => Winding::EvenOdd,
        }
    }
}

impl From<rq_spread_mode> for Spread {
    fn from(value: rq_spread_mode) -> Self {
        match value {
            rq_spread_mode::Pad => Spread::Pad,
            rq_spread_mode::Reflect => Spread::Reflect,
            rq_spread_mode::Repeat => Spread::Repeat,
        }
    }
}

impl From<rq_extend_mode> for ExtendMode {
    fn from(value: rq_extend_mode) -> Self {
        match value {
            rq_extend_mode::Pad => ExtendMode::Pad,
            rq_extend_mode::Repeat => ExtendMode::Repeat,
        }
    }
}

impl From<rq_filter_mode> for FilterMode {
    fn from(value: rq_filter_mode) -> Self {
        match value {
            rq_filter_mode::Nearest => FilterMode::Nearest,
            rq_filter_mode::Bilinear => FilterMode::Bilinear,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rq_blend_mode {
    SourceOver,
    SourceCopy,
    Clear,
    Destination,
    DestinationOver,
    SourceIn,
    DestinationIn,
    SourceOut,
    DestinationOut,
    SourceAtop,
    DestinationAtop,
    Xor,
    Add,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl From<rq_blend_mode> for BlendMode {
    fn from(value: rq_blend_mode) -> Self {
        match value {
            rq_blend_mode::SourceOver => BlendMode::SrcOver,
            rq_blend_mode::SourceCopy => BlendMode::Src,
            rq_blend_mode::Clear => BlendMode::Clear,
            rq_blend_mode::Destination => BlendMode::Dst,
            rq_blend_mode::DestinationOver => BlendMode::DstOver,
            rq_blend_mode::SourceIn => BlendMode::SrcIn,
            rq_blend_mode::DestinationIn => BlendMode::DstIn,
            rq_blend_mode::SourceOut => BlendMode::SrcOut,
            rq_blend_mode::DestinationOut => BlendMode::DstOut,
            rq_blend_mode::SourceAtop => BlendMode::SrcAtop,
            rq_blend_mode::DestinationAtop => BlendMode::DstAtop,
            rq_blend_mode::Xor => BlendMode::Xor,
            rq_blend_mode::Add => BlendMode::Add,
            rq_blend_mode::Multiply => BlendMode::Multiply,
            rq_blend_mode::Screen => BlendMode::Screen,
            rq_blend_mode::Overlay => BlendMode::Overlay,
            rq_blend_mode::Darken => BlendMode::Darken,
            rq_blend_mode::Lighten => BlendMode::Lighten,
            rq_blend_mode::ColorDodge => BlendMode::ColorDodge,
            rq_blend_mode::ColorBurn => BlendMode::ColorBurn,
            rq_blend_mode::HardLight => BlendMode::HardLight,
            rq_blend_mode::SoftLight => BlendMode::SoftLight,
            rq_blend_mode::Difference => BlendMode::Difference,
            rq_blend_mode::Exclusion => BlendMode::Exclusion,
            rq_blend_mode::Hue => BlendMode::Hue,
            rq_blend_mode::Saturation => BlendMode::Saturation,
            rq_blend_mode::Color => BlendMode::Color,
            rq_blend_mode::Luminosity => BlendMode::Luminosity,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rq_cap_style {
    Butt,
    Round,
    Square,
}

impl From<rq_cap_style> for LineCap {
    fn from(value: rq_cap_style) -> Self {
        match value {
            rq_cap_style::Butt => LineCap::Butt,
            rq_cap_style::Round => LineCap::Round,
            rq_cap_style::Square => LineCap::Square,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rq_join_style {
    Miter,
    Round,
    Bevel,
}

impl From<rq_join_style> for LineJoin {
    fn from(value: rq_join_style) -> Self {
        match value {
            rq_join_style::Miter => LineJoin::Miter,
            rq_join_style::Round => LineJoin::Round,
            rq_join_style::Bevel => LineJoin::Bevel,
        }
    }
}

#[repr(C)]
pub struct rq_stroke_style {
    width: f32,
    cap: rq_cap_style,
    join: rq_join_style,
    miter_limit: f32,
    dash_array: *mut f32,
    dash_array_length: usize,
    dash_offset: f32,
}

impl From<&rq_stroke_style> for StrokeStyle {
    fn from(value: &rq_stroke_style) -> Self {
        StrokeStyle {
            width: value.width,
            cap: value.cap.into(),
            join: value.join.into(),
            miter_limit: value.miter_limit,
            dash_array: Vec::new(),
            dash_offset: 0.0,
        }
    }
}

#[repr(C)]
pub struct rq_draw_options {
    alpha: f32,
    blend_mode: rq_blend_mode,
}

impl From<&rq_draw_options> for DrawOptions {
    fn from(value: &rq_draw_options) -> Self {
        DrawOptions {
            alpha: value.alpha,
            blend_mode: value.blend_mode.into(),
            antialias: raqote::AntialiasMode::Gray,
        }
    }
}

pub struct rq_path_builder(PathBuilder);
pub struct rq_path(Path);
pub struct rq_draw_target(DrawTarget);
pub struct rq_argb(Vec<u8>);
pub struct rq_linear_gradient {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    stops: Vec<GradientStop>,
    spread: Spread,
}
pub struct rq_radial_gradient {
    x0: f32,
    y0: f32,
    r0: f32,
    x1: f32,
    y1: f32,
    r1: f32,
    stops: Vec<GradientStop>,
    spread: Spread,
}
pub struct rq_sweep_gradient {
    center_x: f32,
    center_y: f32,
    start_angle: f32,
    end_angle: f32,
    stops: Vec<GradientStop>,
    spread: Spread,
}
pub struct rq_image {
    width: i32,
    height: i32,
    data: Vec<u32>,
}
pub struct rq_pattern {
    image: rq_image,
    extend_mode: ExtendMode,
    filter_mode: FilterMode,
    transform: rq_transform,
}

#[repr(C)]
pub enum rq_paint {
    Solid(rq_color),
    LinearGradient(*mut rq_linear_gradient),
    RadialGradient(*mut rq_radial_gradient),
    SweepGradient(*mut rq_sweep_gradient),
    Pattern(*mut rq_pattern),
}


// Transform functions
#[no_mangle]
pub extern "C" fn rq_transform_identity() -> rq_transform {
    Transform::identity().into()
}

#[no_mangle]
pub extern "C" fn rq_transform_scale(sx: f32, sy: f32) -> rq_transform {
    Transform::scale(sx, sy).into()
}

#[no_mangle]
pub extern "C" fn rq_transform_translate(tx: f32, ty: f32) -> rq_transform {
    Transform::translation(tx, ty).into()
}

#[no_mangle]
pub extern "C" fn rq_transform_rotate(angle: f32) -> rq_transform {
    Transform::rotation(euclid::Angle::radians(angle)).into()
}

#[no_mangle]
pub extern "C" fn rq_transform_multiply(a: rq_transform, b: rq_transform) -> rq_transform {
    let ta: Transform = a.into();
    let tb: Transform = b.into();
    (ta.then(&tb)).into()
}

// Path builder functions
#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_create() -> *mut rq_path_builder {
    Box::into_raw(Box::new(rq_path_builder(PathBuilder::new())))
}

#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_destroy(builder: *mut rq_path_builder) {
    let _ = Box::from_raw(builder);
}

#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_move_to(builder: *mut rq_path_builder, x: f32, y: f32) {
    (*builder).0.move_to(x, y);
}

#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_line_to(builder: *mut rq_path_builder, x: f32, y: f32) {
    (*builder).0.line_to(x, y);
}

#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_quad_to(
    builder: *mut rq_path_builder,
    cx: f32,
    cy: f32,
    x: f32,
    y: f32,
) {
    (*builder).0.quad_to(cx, cy, x, y);
}

#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_cubic_to(
    builder: *mut rq_path_builder,
    cx1: f32,
    cy1: f32,
    cx2: f32,
    cy2: f32,
    x: f32,
    y: f32,
) {
    (*builder).0.cubic_to(cx1, cy1, cx2, cy2, x, y);
}

#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_close(builder: *mut rq_path_builder) {
    (*builder).0.close();
}

#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_arc(
    builder: *mut rq_path_builder,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    sweep_angle: f32,
) {
    (*builder).0.arc(x, y, radius, start_angle, sweep_angle);
}

#[no_mangle]
pub unsafe extern "C" fn rq_path_builder_finish(builder: *mut rq_path_builder) -> *mut rq_path {
    let builder = Box::from_raw(builder);
    let path = builder.0.finish();
    Box::into_raw(Box::new(rq_path(path)))
}

// Path functions
#[no_mangle]
pub unsafe extern "C" fn rq_path_destroy(path: *mut rq_path) {
    let _ = Box::from_raw(path);
}

#[no_mangle]
pub unsafe extern "C" fn rq_rounded_rect(rect: rq_rect, rx: f32, ry: f32) -> *mut rq_path {
    let (x, y, width, height) = (rect.x, rect.y, rect.width, rect.height);

    let rx = rx.min(width / 2.0);
    let ry = ry.min(height / 2.0);

    let mut builder = PathBuilderTracker::new();

    builder.move_to(x + rx, y);

    builder.line_to(x + width - rx, y);
    builder.arc_to(rx, ry, 0.0, false, true, x + width, y + ry);

    builder.line_to(x + width, y + height - ry);
    builder.arc_to(rx, ry, 0.0, false, true, x + width - rx, y + height);

    builder.line_to(x + rx, y + height);
    builder.arc_to(rx, ry, 0.0, false, true, x, y + height - ry);

    builder.line_to(x, y + ry);
    builder.arc_to(rx, ry, 0.0, false, true, x + rx, y);
    
    builder.close();

    Box::into_raw(Box::new(rq_path(builder.finish())))
}

// Helper struct to track position and provide arc_to functionality
struct PathBuilderTracker {
    builder: PathBuilder,
    current_pos: Option<Point>,
}

impl PathBuilderTracker {
    fn new() -> Self {
        Self {
            builder: PathBuilder::new(),
            current_pos: None,
        }
    }
    
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to(x, y);
        self.current_pos = Some(Point::new(x, y));
    }
    
    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(x, y);
        self.current_pos = Some(Point::new(x, y));
    }
    
    fn arc_to(&mut self, rx: f32, ry: f32, x_axis_rotation: f32, large_arc: bool, sweep: bool, x: f32, y: f32) {
        if let Some(from) = self.current_pos {
            let svg_arc = kurbo::SvgArc {
                from: kurbo::Point::new(from.x as f64, from.y as f64),
                to: kurbo::Point::new(x as f64, y as f64),
                radii: kurbo::Vec2::new(rx as f64, ry as f64),
                x_rotation: (x_axis_rotation as f64).to_radians(),
                large_arc,
                sweep,
            };

            match kurbo::Arc::from_svg_arc(&svg_arc) {
                Some(arc) => {
                    arc.to_cubic_beziers(0.1, |p1, p2, p| {
                        self.builder.cubic_to(
                            p1.x as f32,
                            p1.y as f32,
                            p2.x as f32,
                            p2.y as f32,
                            p.x as f32,
                            p.y as f32,
                        );
                    });
                }
                None => {
                    self.builder.line_to(x, y);
                }
            }
            self.current_pos = Some(Point::new(x, y));
        }
    }
    
    fn close(&mut self) {
        self.builder.close();
    }
    
    fn finish(self) -> Path {
        self.builder.finish()
    }
}

// Draw target functions
#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_create(width: i32, height: i32) -> *mut rq_draw_target {
    let dt = DrawTarget::new(width, height);
    Box::into_raw(Box::new(rq_draw_target(dt)))
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_destroy(dt: *mut rq_draw_target) {
    let _ = Box::from_raw(dt);
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_clear(dt: *mut rq_draw_target, color: rq_color) {
    let solid_color: SolidSource = color.into();
    (*dt).0.clear(solid_color);
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_set_transform(dt: *mut rq_draw_target, transform: rq_transform) {
    (*dt).0.set_transform(&transform.into());
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_get_transform(dt: *const rq_draw_target, out_transform: *mut rq_transform) {
    *out_transform = (*(*dt).0.get_transform()).into();
}

// Drawing functions
#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_fill_path(
    dt: *mut rq_draw_target,
    path: *const rq_path,
    paint: rq_paint,
    fill_rule: rq_fill_rule,
    options: *const rq_draw_options,
) {
    let draw_options = if options.is_null() {
        DrawOptions::new()
    } else {
        (&*options).into()
    };
    
    let mut path_with_winding = (*path).0.clone();
    path_with_winding.winding = fill_rule.into();
    
    match paint {
        rq_paint::Solid(color) => {
            let source = Source::Solid(color.into());
            (*dt).0.fill(&path_with_winding, &source, &draw_options);
        },
        rq_paint::LinearGradient(gradient) => {
            let g = &*gradient;
            let start = Point::new(g.x0, g.y0);
            let end = Point::new(g.x1, g.y1);
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_linear_gradient(gradient_data, start, end, g.spread);
            (*dt).0.fill(&path_with_winding, &source, &draw_options);
        },
        rq_paint::RadialGradient(gradient) => {
            let g = &*gradient;
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_two_circle_radial_gradient(
                gradient_data,
                Point::new(g.x0, g.y0),
                g.r0,
                Point::new(g.x1, g.y1),
                g.r1,
                g.spread,
            );
            (*dt).0.fill(&path_with_winding, &source, &draw_options);
        },
        rq_paint::SweepGradient(gradient) => {
            let g = &*gradient;
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_sweep_gradient(
                gradient_data,
                Point::new(g.center_x, g.center_y),
                g.start_angle,
                g.end_angle,
                g.spread,
            );
            (*dt).0.fill(&path_with_winding, &source, &draw_options);
        },
        rq_paint::Pattern(pattern) => {
            let p = &*pattern;
            let raqote_image = Image {
                width: p.image.width,
                height: p.image.height,
                data: &p.image.data,
            };
            let source = Source::Image(raqote_image, p.extend_mode, p.filter_mode, p.transform.into());
            (*dt).0.fill(&path_with_winding, &source, &draw_options);
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_stroke_path(
    dt: *mut rq_draw_target,
    path: *const rq_path,
    paint: rq_paint,
    stroke_style: *const rq_stroke_style,
    options: *const rq_draw_options,
) {
    let draw_options = if options.is_null() {
        DrawOptions::new()
    } else {
        (&*options).into()
    };
    
    let style: StrokeStyle = (&*stroke_style).into();
    
    match paint {
        rq_paint::Solid(color) => {
            let source = Source::Solid(color.into());
            (*dt).0.stroke(&(*path).0, &source, &style, &draw_options);
        },
        rq_paint::LinearGradient(gradient) => {
            let g = &*gradient;
            let start = Point::new(g.x0, g.y0);
            let end = Point::new(g.x1, g.y1);
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_linear_gradient(gradient_data, start, end, g.spread);
            (*dt).0.stroke(&(*path).0, &source, &style, &draw_options);
        },
        rq_paint::RadialGradient(gradient) => {
            let g = &*gradient;
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_two_circle_radial_gradient(
                gradient_data,
                Point::new(g.x0, g.y0),
                g.r0,
                Point::new(g.x1, g.y1),
                g.r1,
                g.spread,
            );
            (*dt).0.stroke(&(*path).0, &source, &style, &draw_options);
        },
        rq_paint::SweepGradient(gradient) => {
            let g = &*gradient;
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_sweep_gradient(
                gradient_data,
                Point::new(g.center_x, g.center_y),
                g.start_angle,
                g.end_angle,
                g.spread,
            );
            (*dt).0.stroke(&(*path).0, &source, &style, &draw_options);
        },
        rq_paint::Pattern(pattern) => {
            let p = &*pattern;
            let raqote_image = Image {
                width: p.image.width,
                height: p.image.height,
                data: &p.image.data,
            };
            let source = Source::Image(raqote_image, p.extend_mode, p.filter_mode, p.transform.into());
            (*dt).0.stroke(&(*path).0, &source, &style, &draw_options);
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_fill_rect(
    dt: *mut rq_draw_target,
    rect: rq_rect,
    paint: rq_paint,
    options: *const rq_draw_options,
) {
    let draw_options = if options.is_null() {
        DrawOptions::new()
    } else {
        (&*options).into()
    };
    
    match paint {
        rq_paint::Solid(color) => {
            let source = Source::Solid(color.into());
            (*dt).0.fill_rect(rect.x, rect.y, rect.width, rect.height, &source, &draw_options);
        },
        rq_paint::LinearGradient(gradient) => {
            let g = &*gradient;
            let start = Point::new(g.x0, g.y0);
            let end = Point::new(g.x1, g.y1);
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_linear_gradient(gradient_data, start, end, g.spread);
            (*dt).0.fill_rect(rect.x, rect.y, rect.width, rect.height, &source, &draw_options);
        },
        rq_paint::RadialGradient(gradient) => {
            let g = &*gradient;
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_two_circle_radial_gradient(
                gradient_data,
                Point::new(g.x0, g.y0),
                g.r0,
                Point::new(g.x1, g.y1),
                g.r1,
                g.spread,
            );
            (*dt).0.fill_rect(rect.x, rect.y, rect.width, rect.height, &source, &draw_options);
        },
        rq_paint::SweepGradient(gradient) => {
            let g = &*gradient;
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_sweep_gradient(
                gradient_data,
                Point::new(g.center_x, g.center_y),
                g.start_angle,
                g.end_angle,
                g.spread,
            );
            (*dt).0.fill_rect(rect.x, rect.y, rect.width, rect.height, &source, &draw_options);
        },
        rq_paint::Pattern(pattern) => {
            let p = &*pattern;
            let raqote_image = Image {
                width: p.image.width,
                height: p.image.height,
                data: &p.image.data,
            };
            let source = Source::Image(raqote_image, p.extend_mode, p.filter_mode, p.transform.into());
            (*dt).0.fill_rect(rect.x, rect.y, rect.width, rect.height, &source, &draw_options);
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_stroke_rect(
    dt: *mut rq_draw_target,
    rect: rq_rect,
    paint: rq_paint,
    stroke_style: *const rq_stroke_style,
    options: *const rq_draw_options,
) {
    let draw_options = if options.is_null() {
        DrawOptions::new()
    } else {
        (&*options).into()
    };
    
    // Create a path for the rectangle
    let mut builder = PathBuilder::new();
    builder.move_to(rect.x, rect.y);
    builder.line_to(rect.x + rect.width, rect.y);
    builder.line_to(rect.x + rect.width, rect.y + rect.height);
    builder.line_to(rect.x, rect.y + rect.height);
    builder.close();
    let path = builder.finish();
    
    let style: StrokeStyle = (&*stroke_style).into();
    
    match paint {
        rq_paint::Solid(color) => {
            let source = Source::Solid(color.into());
            (*dt).0.stroke(&path, &source, &style, &draw_options);
        },
        rq_paint::LinearGradient(gradient) => {
            let g = &*gradient;
            let start = Point::new(g.x0, g.y0);
            let end = Point::new(g.x1, g.y1);
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_linear_gradient(gradient_data, start, end, g.spread);
            (*dt).0.stroke(&path, &source, &style, &draw_options);
        },
        rq_paint::RadialGradient(gradient) => {
            let g = &*gradient;
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_two_circle_radial_gradient(
                gradient_data,
                Point::new(g.x0, g.y0),
                g.r0,
                Point::new(g.x1, g.y1),
                g.r1,
                g.spread,
            );
            (*dt).0.stroke(&path, &source, &style, &draw_options);
        },
        rq_paint::SweepGradient(gradient) => {
            let g = &*gradient;
            let gradient_data = Gradient {
                stops: g.stops.clone(),
            };
            let source = Source::new_sweep_gradient(
                gradient_data,
                Point::new(g.center_x, g.center_y),
                g.start_angle,
                g.end_angle,
                g.spread,
            );
            (*dt).0.stroke(&path, &source, &style, &draw_options);
        },
        rq_paint::Pattern(pattern) => {
            let p = &*pattern;
            let raqote_image = Image {
                width: p.image.width,
                height: p.image.height,
                data: &p.image.data,
            };
            let source = Source::Image(raqote_image, p.extend_mode, p.filter_mode, p.transform.into());
            (*dt).0.stroke(&path, &source, &style, &draw_options);
        },
    }
}

// Pixel data access
#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_get_data(dt: *mut rq_draw_target) -> *mut rq_argb {
    let data = (*dt).0.get_data();
    let mut buffer = Vec::with_capacity(data.len() * 4);
    
    // Convert from raqote format to BGRA format expected by blend2d
    for pixel in data {
        let a = (pixel >> 24) & 0xff;
        let r = (pixel >> 16) & 0xff;
        let g = (pixel >> 8) & 0xff;
        let b = pixel & 0xff;
        
        buffer.push(b as u8);  // B
        buffer.push(g as u8);  // G  
        buffer.push(r as u8);  // R
        buffer.push(a as u8);  // A
    }
    
    Box::into_raw(Box::new(rq_argb(buffer)))
}

#[no_mangle]
pub unsafe extern "C" fn rq_argb_data(data: *const rq_argb) -> *const u8 {
    (*data).0.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn rq_argb_destroy(data: *mut rq_argb) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_width(dt: *const rq_draw_target) -> i32 {
    (*dt).0.width()
}

#[no_mangle]
pub unsafe extern "C" fn rq_draw_target_height(dt: *const rq_draw_target) -> i32 {
    (*dt).0.height()
}

// Gradient functions
#[no_mangle]
pub unsafe extern "C" fn rq_linear_gradient_create(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    spread: rq_spread_mode,
) -> *mut rq_linear_gradient {
    Box::into_raw(Box::new(rq_linear_gradient {
        x0,
        y0,
        x1,
        y1,
        stops: Vec::new(),
        spread: spread.into(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn rq_radial_gradient_create(
    x0: f32,
    y0: f32,
    r0: f32,
    x1: f32,
    y1: f32,
    r1: f32,
    spread: rq_spread_mode,
) -> *mut rq_radial_gradient {
    Box::into_raw(Box::new(rq_radial_gradient {
        x0,
        y0,
        r0,
        x1,
        y1,
        r1,
        stops: Vec::new(),
        spread: spread.into(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn rq_linear_gradient_add_stop(
    gradient: *mut rq_linear_gradient,
    stop: rq_gradient_stop,
) {
    (*gradient).stops.push(stop.into());
}

#[no_mangle]
pub unsafe extern "C" fn rq_radial_gradient_add_stop(
    gradient: *mut rq_radial_gradient,
    stop: rq_gradient_stop,
) {
    (*gradient).stops.push(stop.into());
}

#[no_mangle]
pub unsafe extern "C" fn rq_linear_gradient_destroy(gradient: *mut rq_linear_gradient) {
    let _ = Box::from_raw(gradient);
}

#[no_mangle]
pub unsafe extern "C" fn rq_radial_gradient_destroy(gradient: *mut rq_radial_gradient) {
    let _ = Box::from_raw(gradient);
}

#[no_mangle]
pub unsafe extern "C" fn rq_sweep_gradient_create(
    center_x: f32,
    center_y: f32,
    start_angle: f32,
    end_angle: f32,
    spread: rq_spread_mode,
) -> *mut rq_sweep_gradient {
    Box::into_raw(Box::new(rq_sweep_gradient {
        center_x,
        center_y,
        start_angle,
        end_angle,
        stops: Vec::new(),
        spread: spread.into(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn rq_sweep_gradient_add_stop(
    gradient: *mut rq_sweep_gradient,
    stop: rq_gradient_stop,
) {
    (*gradient).stops.push(stop.into());
}

#[no_mangle]
pub unsafe extern "C" fn rq_sweep_gradient_destroy(gradient: *mut rq_sweep_gradient) {
    let _ = Box::from_raw(gradient);
}

// Pattern functions  
#[no_mangle]
pub unsafe extern "C" fn rq_image_create(width: i32, height: i32, data: *const u32) -> *mut rq_image {
    let size = (width * height) as usize;
    let data_slice = std::slice::from_raw_parts(data, size);
    
    Box::into_raw(Box::new(rq_image {
        width,
        height,
        data: data_slice.to_vec(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn rq_image_destroy(image: *mut rq_image) {
    let _ = Box::from_raw(image);
}

#[no_mangle]
pub unsafe extern "C" fn rq_pattern_create(
    image: *mut rq_image,
    extend_mode: rq_extend_mode,
    filter_mode: rq_filter_mode,
    transform: rq_transform,
) -> *mut rq_pattern {
    Box::into_raw(Box::new(rq_pattern {
        image: std::ptr::read(image),
        extend_mode: extend_mode.into(),
        filter_mode: filter_mode.into(),
        transform,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn rq_pattern_destroy(pattern: *mut rq_pattern) {
    let _ = Box::from_raw(pattern);
}

// Paint helper functions
#[no_mangle]
pub unsafe extern "C" fn rq_paint_destroy(paint: rq_paint) {
    match paint {
        rq_paint::Solid(_) => {
            // Nothing to clean up for solid colors
        },
        rq_paint::LinearGradient(_) => {
            // Gradients are managed externally, don't destroy here
        },
        rq_paint::RadialGradient(_) => {
            // Gradients are managed externally, don't destroy here
        },
        rq_paint::SweepGradient(_) => {
            // Gradients are managed externally, don't destroy here
        },
        rq_paint::Pattern(_) => {
            // Patterns are managed externally, don't destroy here
        },
    }
}