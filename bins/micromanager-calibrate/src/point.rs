use rand::Rng;

pub type Point = (f64, f64, f64);

pub struct PointCloud {}

impl PointCloud {
    pub fn iter() -> Self {
        Self {}
    }
}

impl Iterator for PointCloud {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();

        let mut get_point = || loop {
            let num = (rng.gen::<f64>() - 0.5) * 2.0;

            if num == 0.0 {
                continue;
            }

            break num;
        };

        let x = get_point();
        let y = get_point();
        let z = get_point();

        let mag = (x * x + y * y + z * z).sqrt();

        Some((x / mag, y / mag, z / mag))
    }
}
