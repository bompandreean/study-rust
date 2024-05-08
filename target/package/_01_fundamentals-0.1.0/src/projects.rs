pub fn project1() {
    const EARTH_RADIUS_IN_KM: f64 = 6371.0;

    let kcle_latitude_degrees: f64 = 41.4075;
    let kcle_longitude_degrees: f64 = -81.85111;

    let kcle_latitude_radians = kcle_latitude_degrees.to_radians();
    let kcle_longitude_radians = kcle_longitude_degrees.to_radians();
    println!("{}, {}", kcle_longitude_radians, kcle_latitude_radians);

    let ksle_lat_degrees: f64 = 40.7861;
    let ksle_long_degrees: f64 = -111.9822;

    let delta_lat = (kcle_latitude_degrees - ksle_lat_degrees).to_radians();
    let delta_long = (kcle_longitude_degrees - ksle_long_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_lat / 2.0).sin(), 2)
        + kcle_latitude_radians.cos() * ksle_lat_degrees.to_radians().cos()
        * f64::powi((delta_long / 2.0).sin(), 2);

    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
    let distance = EARTH_RADIUS_IN_KM * central_angle;
    println!("The distance between the two point is {:.1} kilometers", distance);
}


pub fn project2() {
    const EARTH_RADIUS_IN_KM: f64 = 6371.0;

    let locations = [("kcle", 41.4075, -81.85111),
        ("ksle", 40.78614, -111.9822),
        ("la", -89.14, 23.87)];

    let mut total_distance = 0.00;
    let mut prev_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in locations {
        match prev_waypoint {
            None => {
                prev_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(prev_waypoint_value) => {
                let prev_waypoint_radius: f64 = prev_waypoint_value.1.to_radians();
                let current_waypoint_radius: f64 = waypoint.1.to_radians();

                let delta_lat: f64 = (prev_waypoint_value.1 - waypoint.1).to_radians();
                let delta_long: f64 = (prev_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_lat / 2.0).sin(), 2)
                    + prev_waypoint_radius.cos() * current_waypoint_radius.cos()
                    * f64::powi((delta_long / 2.0).sin(), 2);

                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KM * central_angle;
                total_distance += distance;
                println!("The distance between {} and {} point is {:.1} kilometers", prev_waypoint_value.0, waypoint.0, distance);
                prev_waypoint = Option::from(waypoint.clone());
            }
        }
    }

    println!("The distance between all the points is {:.1} kilometers", total_distance);
}
