#[derive(Debug)]
struct Hailstone {
    x: f32,
    y: f32,
    z: f32,
    vel_x: f32,
    vel_y: f32,
    vel_z: f32,
}

pub fn day24_1(text: &str, zone_min: f32, zone_max: f32) -> u32 {
    // How many hailstone paths cross within the zone? Ignore z for part one
    let hail = build_hailstones(text);
    let mut count_intersections = 0;

    // Solve system of equations to find intersection, if slopes not parallel
    for i in 0..hail.len() - 1 {
        let slope1 = hail[i].vel_y / hail[i].vel_x;
        let intercept1 = hail[i].y - slope1 * hail[i].x;
        for j in (i + 1)..hail.len() {
            let slope2 = hail[j].vel_y / hail[j].vel_x;
            let intercept2 = hail[j].y - slope2 * hail[j].x;
            if slope1 == slope2 {
                continue;
            }
            let x_meet = (intercept1 - intercept2) / (slope2 - slope1);
            let y_meet = slope1 * x_meet + intercept1;
            // Determine if the intersection is in the path of both rays
            let mut will_intersect = true;
            for (slope, stone) in [(slope1, &hail[i]), (slope2, &hail[j])] {
                if slope >= 0.0 {
                    if stone.vel_x < 0.0 && stone.vel_y < 0.0 {
                        if x_meet > stone.x || y_meet > stone.y {
                            will_intersect = false;
                            break;
                        }
                    } else if x_meet < stone.x || y_meet < stone.y {
                        will_intersect = false;
                        break;
                    }
                } else if slope < 0.0 {
                    if stone.vel_x < 0.0 {
                        if x_meet > stone.x {
                            will_intersect = false;
                            break;
                        }
                    } else if stone.vel_y < 0.0 {
                        if y_meet > stone.y {
                            will_intersect = false;
                            break;
                        }
                    } else {
                        panic!();
                    }
                }
            }
            if !will_intersect {
                continue;
            }

            if zone_min <= x_meet && x_meet <= zone_max && zone_min <= y_meet && y_meet <= zone_max
            {
                count_intersections += 1;
            }
        }
    }

    return count_intersections;
}

fn build_hailstones(text: &str) -> Vec<Hailstone> {
    let mut hail = vec![];
    for line in text.lines() {
        let (locations, velocities) = line.split_once(" @ ").unwrap();
        let locations = locations
            .split(", ")
            .map(|s| s.trim())
            .map(|s| s.parse::<f32>().unwrap_or_default())
            .collect::<Vec<f32>>();
        let velocities = velocities
            .split(", ")
            .map(|s| s.trim())
            .map(|s| s.parse::<f32>().unwrap_or_default())
            .collect::<Vec<f32>>();
        hail.push(Hailstone {
            x: locations[0],
            y: locations[1],
            z: locations[2],
            vel_x: velocities[0],
            vel_y: velocities[1],
            vel_z: velocities[2],
        });
    }

    return hail;
}
