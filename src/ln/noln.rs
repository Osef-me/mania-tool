use rosu_map::Beatmap;
use rosu_map::section::hit_objects::{HitObjectCircle, HitObjectKind};
use rosu_map::util::Pos;

/// Remove all long notes (converts holds to regular notes)
pub fn noln(map : &mut Beatmap) {

    for hit_object in map.hit_objects.iter_mut() {
        if let HitObjectKind::Hold(hold) = hit_object.kind {
            hit_object.kind = HitObjectKind::Circle(HitObjectCircle {
                pos: Pos {
                    x: hold.pos_x,
                    y: 0.0,
                },
                new_combo: false,
                combo_offset: 0,
            });
        }
    }

    map.version = format!("{} No LN", map.version);
}

