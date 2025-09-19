use rosu_map::{Beatmap};
use rosu_map::section::hit_objects::HitObject;
use rosu_map::section::hit_objects::HitObjectKind;

pub fn rate(rate: f64, map: &mut Beatmap) {
    map.audio_file = map.audio_file.replace(".mp3", format!("_r{}.ogg", rate).as_str());
    let time_multiplier: f64 = 1.0 / rate;

    for hit_object in &mut map.hit_objects {
        match_hit_object(hit_object, time_multiplier);
    }

    for timing_point in &mut map.control_points.timing_points {
        timing_point.time *= time_multiplier;
        timing_point.beat_len *= time_multiplier;
    }

    for effect_point in &mut map.control_points.effect_points {
        effect_point.time *= time_multiplier;
    }

    for difficulty_point in &mut map.control_points.difficulty_points {
        difficulty_point.time *= time_multiplier;
    }

    for point in &mut map.control_points.sample_points {
        point.time *= time_multiplier;
    }

    map.version.push_str(&format!(" x{}", rate));
}

fn match_hit_object(hit_object: &mut HitObject, time_multiplier: f64) {
    hit_object.start_time *= time_multiplier;
    if let HitObjectKind::Hold(hold) = &mut hit_object.kind {
        hold.duration *= time_multiplier;
    }
}