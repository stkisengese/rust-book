pub use crate::areas_volumes::*;
pub mod areas_volumes;

pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
    let total_area = x * y;
    let required_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a) * times,
        GeometricalShapes::Circle => (areas_volumes::circle_area(a) * times as f64) as usize,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) * times,
        GeometricalShapes::Triangle => (areas_volumes::triangle_area(a, b) * times as f64) as usize,
    };
    total_area >= required_area

}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    let total_volume = x * y * z;
    let required_volume = match objects {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) * times,
        GeometricalVolumes::Sphere => (areas_volumes::sphere_volume(a) * times as f64) as usize,
        GeometricalVolumes::Cone => (areas_volumes::cone_volume(a, b) * times as f64) as usize,
        GeometricalVolumes::Pyramid => (areas_volumes::triangular_pyramid_volume(a as f64, b) * times as f64) as usize,
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) * times,
    };
    total_volume >= required_volume
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = area_fit(2, 4, GeometricalShapes::Rectangle, 100, 2, 1);
        assert_eq!(result, true);
    }
}
