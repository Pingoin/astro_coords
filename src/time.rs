use chrono::prelude::*;

use crate::angle::Angle;

pub fn get_greenwich_mean_siderial_time(time: DateTime<Utc>) -> Angle {
    let t = (get_julian_date(time) - 2451545.0) / 36525.0;
    let theta0 =
        280.46061837 + 36000.770053608 * t + (0.000387933 * t * t) - (t * t * t / 38710000.0);
    Angle::from_degree(theta0)
}

pub fn get_julian_date(time: DateTime<Utc>) -> f32 {
    (365.25f32 * ((time.year() as f32) + 4716f32)).floor()
        + (30.6001 * ((time.month() as f32) + 1f32)).floor()
        + (time.day() as f32
            + time.hour() as f32 / 24.0
            + time.minute() as f32 / 1440.0
            + time.second() as f32 / 86400.0
        )
        + (2f32 - ((time.year() as f32) / 100.0).floor() + ((time.year() as f32) / 400.0).floor())
        - 1524.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_date() {
        let t1 = "1582-10-15T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let jd1 = get_julian_date(t1);
        assert_eq!(jd1, 2_299_160.500);
    }

  /*  #[test]
    fn test_gmst(){
        let time = "2020-03-20T03:50:00Z".parse::<DateTime<Utc>>().unwrap();
        println!("{}",get_greenwich_mean_siderial_time(time).to_degree());
    }*/
}
