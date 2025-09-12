mod areas_volumes;
use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize)
) -> bool {
    let area = x * y;
    match kind {
        Square => square_area(a ) * times < area,
        Circle => circle_area(a) * times as f64 < area as  f64,
        Rectangle => rectangle_area(a, b) * times as f64< area,
        Triangle => triangle_area(a, b) * times as  f64 < area as f64,
        _ => false,
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize)
) -> bool {
    let volume = x*y*z;
    match  {
        Cube => cube_volume(a)*times<volume,
        Sphere => sphere_volume(a)*times<volume,
        Cone => cone_volume(a, b)*times<volume,
        TriangularPyramid => triangular_pyramid_volume(a as f64, b as f64,c as f64)*times as f64<volume as f64,
        Parallelepiped => parallelepiped_volume(a as f64,b as f64)*times as f64<volume as f64,
        _=> false ,
    }
}
