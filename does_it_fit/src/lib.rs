mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize)
) -> bool {
    let area = x * y;
    match kind {
        GeometricalShapes::Square => square_area(a) * times < area,
        GeometricalShapes::Circle => circle_area(a) * (times as f64) < ({ area as f64 }),
        GeometricalShapes::Rectangle => rectangle_area(a, b) * times < area,
        GeometricalShapes::Triangle => triangle_area(a, b) * (times as f64) < ({ area as f64 }),
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize)
) -> bool {
    let volume = x * y * z;
    match kind {
        GeometricalVolumes::Cube => cube_volume(a) * times < volume,
        GeometricalVolumes::Sphere => sphere_volume(a) * (times as f64) < volume as f64,
        GeometricalVolumes::Cone => cone_volume(a, b) *(times as f64) < volume as f64,
        GeometricalVolumes::TriangularPyramid =>
            triangular_pyramid_volume(a as f64, b) * (times as f64) <
                (volume as f64),
        GeometricalVolumes::Parallelepiped =>
            parallelepiped_volume(a , b ,c) * times < volume,
    }
}
