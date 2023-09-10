use iced::Point;
use std::f32::consts::PI;

const EPS: f32 = 1.0E-6;
type Angle = f32;

#[derive(Default, Copy, Clone)]
pub struct Ellipse {
    pub center: Point,
    pub major_curvature: f32,
    pub minor_radius: f32,
    pub angle: Angle,
}

impl Ellipse {
    pub fn round(r: f32) -> Self {
        Ellipse {
            center: Point::ORIGIN,
            major_curvature: 1.0 / r,
            minor_radius: r,
            angle: 0.0,
        }
    }

    pub fn get_point(&self, angle: Angle) -> Point {
        let pt = self.get_relative_point(angle);
        Point::new(self.center.x + pt.0, self.center.y + pt.1)
    }

    fn get_relative_point(&self, angle: Angle) -> (f32, f32) {
        println!("get_relative_point {angle}");
        if self.major_curvature.abs() < EPS {
            let beta = angle + self.angle;
            let cos_gamma = (PI / 2.0 + self.angle - beta).cos().abs();
            if cos_gamma < EPS {
                panic!("constrait error @ {angle} : {cos_gamma} < {EPS}");
            }
            let x = self.minor_radius * beta.cos() / cos_gamma;
            let y = self.minor_radius * beta.sin() / cos_gamma;
            (x, y)
        } else {
            let x = angle.cos() * self.angle.cos() / self.major_curvature
                - angle.sin() * self.angle.sin() * self.minor_radius;
            let y = angle.cos() * self.angle.sin() / self.major_curvature
                + angle.sin() * self.angle.cos() * self.minor_radius;
            (x, y)
        }
    }
}

#[test]
fn get_point() {
    let radius = 200.0;
    let outer = Ellipse {
        center: Point::ORIGIN,
        major_curvature: 1.0 / radius,
        minor_radius: radius,
        angle: 0.0,
    };
    let t = outer.get_point(0.0);
    println!("{t:?}");
}
