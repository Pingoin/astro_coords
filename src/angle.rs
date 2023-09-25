use core::f32::consts::PI;
const DEG_TO_ARC: f32 = PI / 180f32;

/// Helperstruct for handing angles
///
/// # Examples
///
/// Create an Angle with a known degree value
/// ```
///  use astro_coords::angle::Angle;
///  use core::f32::consts::PI;
///
///  let angle = Angle::from_degree(90f32);
///  assert_eq!(angle.to_arc(), PI / 2f32);
///  assert_eq!(angle.to_degree(), 90f32);
///
/// ```
///
/// Create an Angle with a known arc value
/// ```
///  use astro_coords::angle::Angle;
///  use core::f32::consts::PI;
///
///  let angle = Angle::from_arc(PI);
///  assert_eq!(angle.to_degree().clone(), 180f32);
///  assert_eq!(angle.to_arc(), PI);
///
/// ```
pub struct Angle {
    arc: f32,
}

impl Angle {
    /// Creates an Angle Object from an Degree Value
    pub fn from_degree(val: f32) -> Angle {
        Angle {
            arc: val * DEG_TO_ARC,
        }
    }
    /// Returns the Angle as degree
    pub fn to_degree(&self) -> f32 {
        self.arc / DEG_TO_ARC
    }
    /// creates the Angle Object from arc value
    pub fn from_arc(val: f32) -> Angle {
        Angle { arc: val }
    }
    /// returns the arc value
    pub fn to_arc(&self) -> f32 {
        self.arc
    }

    /// returns the angle as hour angle
    pub fn to_hour(&self) -> f32 {
        self.arc / PI * 12f32
    }

    /// normalizes the Angle in a reagion between 0-360°
    pub fn normalize(&mut self) {
        let mut degree = self.to_degree() % 360f32;

        if degree < 0.0 {
            degree += 360.0;
        }

        *self = Angle::from_degree(degree);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_calculations() {
        let angle = Angle::from_arc(PI);
        assert_eq!(angle.to_degree().clone(), 180f32);
        assert_eq!(angle.to_arc(), PI);
    }

    #[test]
    fn test_degree_calculations() {
        let angle = Angle::from_degree(90f32);
        assert_eq!(angle.to_arc(), PI / 2f32);
        assert_eq!(angle.to_degree(), 90f32);
    }

    #[test]
    fn test_hour_angles() {
        let angle = Angle::from_degree(90f32);
        assert_eq!(angle.to_hour(), 6f32);
    }
    #[test]
    fn test_normalize() {
        let mut angle = Angle::from_degree(-1f32);
        angle.normalize();
        assert_eq!(angle.to_degree(), 359f32);
        let mut angle = Angle::from_degree(400f32);
        angle.normalize();
        assert_eq!(angle.to_degree(), 40f32);
        let mut angle = Angle::from_degree(721f32);
        angle.normalize();
        assert_eq!(angle.to_degree(), 1f32);
    }
}
