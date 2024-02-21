#[derive(Debug)]
struct Hailstone {
    x: f32,
    y: f32,
    z: f32,
    vel_x: f32,
    vel_y: f32,
    vel_z: f32,
}

pub fn day24_1(text: &str) -> u32 {
    // How many hailstone paths cross within the zone? Ignore z for part one
    // example test zone: 7 <= x, y <= 27
    let hail = build_hailstones(text);

    // TODO: solve system of equations to find intersection, if slopes not identical
    // TODO: determine if intersection will happen in the future (include) or the past (exclude)
    for i in 0..hail.len() - 1 {
        let slope1 = hail[i].vel_y / hail[i].vel_x;
        let intercept1 = hail[i].y - slope1 * hail[i].x;
        println!("y1 = {slope1}x + {intercept1}");
        for j in (i + 1)..hail.len() {
            let slope2 = hail[j].vel_y / hail[j].vel_x;
            let intercept2 = hail[j].y - slope2 * hail[j].x;
            //println!("y2 = {slope2}x + {intercept2}");
        }
    }

    return 0;
}

fn build_hailstones(text: &str) -> Vec<Hailstone> {
    let mut hail = vec![];
    for line in text.lines() {
        let (mut locations, mut velocities) = line.split_once(" @ ").unwrap();
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
