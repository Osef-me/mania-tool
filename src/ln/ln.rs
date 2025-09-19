use rosu_map::Beatmap;
use rosu_map::section::hit_objects::{HitObjectHold, HitObjectKind};

pub fn full_ln(map: &mut Beatmap, gap_ms: f64, duration_warn: f64) {
    map.hit_objects.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());

    let mut transformations: Vec<(usize, f64)> = Vec::new();
    let mut column_end_times: Vec<f64> = vec![0.0; 4];

    let mut i = 0;
    while i < map.hit_objects.len() {
        let hit_object = &map.hit_objects[i];

        if let HitObjectKind::Circle(circle) = hit_object.kind {
            let column = (circle.pos.x / 512.0 * 4.0).floor() as usize;
            let column = column.min(3);

            if hit_object.start_time >= column_end_times[column] {
                let mut duration = gap_ms;

                for j in (i + 1)..map.hit_objects.len() {
                    let next_obj = &map.hit_objects[j];
                    let next_column = match next_obj.kind {
                        HitObjectKind::Circle(ref c) => (c.pos.x / 512.0 * 4.0).floor() as usize,
                        HitObjectKind::Hold(ref h) => (h.pos_x / 512.0 * 4.0).floor() as usize,
                        _ => continue,
                    };

                    if next_column == column {
                        duration = (next_obj.start_time - hit_object.start_time) - gap_ms;
                        if duration < 0.0 {
                            duration = 0.0;
                        }
                        break;
                    }
                }

                if duration > duration_warn {
                    if hit_object.start_time + duration > column_end_times[column] {
                        transformations.push((i, duration));
                        column_end_times[column] = hit_object.start_time + duration;
                    }
                }
            }
        }
        i += 1;
    }

    for (index, duration) in transformations {
        if let HitObjectKind::Circle(circle) = map.hit_objects[index].kind {
            map.hit_objects[index].kind = HitObjectKind::Hold(HitObjectHold {
                pos_x: circle.pos.x,
                duration,
            });
        }
    }

    map.version = format!("{} Full LN", map.version);
}


pub fn full_ln_beat_divisor(map: &mut Beatmap, beat_divisor: f64, warn_duration: Option<f64>) {
    let tpoints = map.control_points.timing_points.iter()
        .max_by(|a, b| a.beat_len.partial_cmp(&b.beat_len).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
    let gap = tpoints.beat_len*beat_divisor;
    let warn_duration = warn_duration.unwrap_or(gap);
    full_ln(map, gap, warn_duration);
}

