struct GenIndex {
    gen: u32,
    index: usize,
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}
struct PointId(GenIndex);

struct Knot(f64);
struct KnotId(GenIndex);

struct Thickness(f64);
struct ThicknessId(GenIndex);

struct Color {
    r: u8,
    g: u8,
    b: u8,
}
struct ColorId(GenIndex);

struct Curve {
    points: Vec<PointId>,
    knots: Vec<KnotId>,
}
struct CurveId(GenIndex);

/// Could be BulkCurveView with curves.len() of 1. Showing that there
/// can be multiple view types for a conceptual curve. You could have another view with no Thickness value or whatever
struct CurveView {
    curve: CurveId,
    thickness: ThicknessId,
    color: ColorId,
}

struct BulkCurveView {
    curves: Vec<CurveId>,
    thickness: ThicknessId,
    color: ColorId,
}

struct Surface {
    curves: Vec<CurveId>,
    knots: Vec<KnotId>,
}

struct World {
    points: Vec<Option<Point>>,
    knots: Vec<Option<Knot>>,
    thickness: Vec<Option<Thickness>>,
    curves: Vec<Option<Curve>>,
    curve_views: Vec<Option<CurveView>>,
    bulk_curve_views: Vec<Option<BulkCurveView>>,
    surfaces: Vec<Option<Surface>>,
}