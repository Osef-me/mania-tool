use rosu_map::Beatmap;

/// Concatenates multiple beatmaps into a single marathon beatmap
/// 
/// # Arguments
/// * `beatmaps` - A vector of beatmaps to concatenate
/// * `gap_ms` - Wait time between each beatmap in milliseconds (optional, default: 0.0)
/// 
/// # Returns
/// A new beatmap containing all elements from the input beatmaps
pub fn concat_beatmaps(beatmaps: Vec<Beatmap>, gap_ms: Option<f64>) -> Beatmap {
    if beatmaps.is_empty() {
        panic!("Cannot concatenate empty vector of beatmaps");
    }

    let gap_ms = gap_ms.unwrap_or(0.0);
    let mut result = beatmaps[0].clone();
    
    // If we only have one beatmap, return it as is
    if beatmaps.len() == 1 {
        return result;
    }

    // Calculate the total duration of the first beatmap
    let mut current_time_offset = get_beatmap_duration(&result) + gap_ms;

    // Concatenate the remaining beatmaps
    for (_i, beatmap) in beatmaps.iter().enumerate().skip(1) {
        // Concatenate hit objects
        for mut hit_object in beatmap.hit_objects.clone() {
            hit_object.start_time += current_time_offset;
            result.hit_objects.push(hit_object);
        }

        // Concatenate timing points
        for mut timing_point in beatmap.control_points.timing_points.clone() {
            timing_point.time += current_time_offset;
            result.control_points.timing_points.push(timing_point);
        }

        // Concatenate effect points
        for mut effect_point in beatmap.control_points.effect_points.clone() {
            effect_point.time += current_time_offset;
            result.control_points.effect_points.push(effect_point);
        }

        // Concatenate difficulty points
        for mut difficulty_point in beatmap.control_points.difficulty_points.clone() {
            difficulty_point.time += current_time_offset;
            result.control_points.difficulty_points.push(difficulty_point);
        }

        // Concatenate sample points
        for mut sample_point in beatmap.control_points.sample_points.clone() {
            sample_point.time += current_time_offset;
            result.control_points.sample_points.push(sample_point);
        }

        // Update the time offset for the next beatmap
        current_time_offset += get_beatmap_duration(beatmap) + gap_ms;
    }

    // Sort all control points by time
    result.control_points.timing_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    result.control_points.effect_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    result.control_points.difficulty_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    result.control_points.sample_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    // Sort hit objects by time
    result.hit_objects.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());

    // Update metadata
    result.version = format!("{} Marathon ({} maps)", result.version, beatmaps.len());

    result
}

/// Calculates the duration of a beatmap in milliseconds
fn get_beatmap_duration(beatmap: &Beatmap) -> f64 {
    if beatmap.hit_objects.is_empty() {
        return 0.0;
    }

    let mut max_time: f64 = 0.0;
    
    for hit_object in &beatmap.hit_objects {
        let end_time = match hit_object.kind {
            rosu_map::section::hit_objects::HitObjectKind::Hold(ref hold) => {
                hit_object.start_time + hold.duration
            },
            _ => hit_object.start_time,
        };
        
        max_time = max_time.max(end_time);
    }
    
    max_time
}

/// Concatenates multiple beatmaps with custom transitions
/// 
/// # Arguments
/// * `beatmaps` - A vector of beatmaps to concatenate
/// * `transitions` - A vector of transition times between each beatmap (optional)
/// 
/// # Returns
/// A new beatmap containing all elements from the input beatmaps
pub fn concat_beatmaps_with_transitions(beatmaps: Vec<Beatmap>, transitions: Option<Vec<f64>>) -> Beatmap {
    if beatmaps.is_empty() {
        panic!("Cannot concatenate empty vector of beatmaps");
    }

    let transitions = transitions.unwrap_or_else(|| vec![0.0; beatmaps.len() - 1]);
    
    if transitions.len() != beatmaps.len() - 1 {
        panic!("Number of transitions must be one less than number of beatmaps");
    }

    let mut result = beatmaps[0].clone();
    
    if beatmaps.len() == 1 {
        return result;
    }

    let mut current_time_offset = get_beatmap_duration(&result);

    for (_i, beatmap) in beatmaps.iter().enumerate().skip(1) {
        // Add the transition
        current_time_offset += transitions[_i - 1];

        // Concatenate beatmap elements
        for mut hit_object in beatmap.hit_objects.clone() {
            hit_object.start_time += current_time_offset;
            result.hit_objects.push(hit_object);
        }

        for mut timing_point in beatmap.control_points.timing_points.clone() {
            timing_point.time += current_time_offset;
            result.control_points.timing_points.push(timing_point);
        }

        for mut effect_point in beatmap.control_points.effect_points.clone() {
            effect_point.time += current_time_offset;
            result.control_points.effect_points.push(effect_point);
        }

        for mut difficulty_point in beatmap.control_points.difficulty_points.clone() {
            difficulty_point.time += current_time_offset;
            result.control_points.difficulty_points.push(difficulty_point);
        }

        for mut sample_point in beatmap.control_points.sample_points.clone() {
            sample_point.time += current_time_offset;
            result.control_points.sample_points.push(sample_point);
        }

        current_time_offset += get_beatmap_duration(beatmap);
    }

    // Sort all elements by time
    result.control_points.timing_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    result.control_points.effect_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    result.control_points.difficulty_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    result.control_points.sample_points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    result.hit_objects.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());

    result.version = format!("{} Marathon ({} maps)", result.version, beatmaps.len());

    result
}
