pub const UNICODE_VERSION: f32 = 13.1f32;
pub const UNICODE_RELEASE_TIME: &'static str = "2020-08-28T05:24:13+00:00";
pub mod smileys_and_emotion {
    #[path = "emoji_subgroup_cat_face.rs"]
    pub mod cat_face;
    #[path = "emoji_subgroup_emotion.rs"]
    pub mod emotion;
    #[path = "emoji_subgroup_face_affection.rs"]
    pub mod face_affection;
    #[path = "emoji_subgroup_face_concerned.rs"]
    pub mod face_concerned;
    #[path = "emoji_subgroup_face_costume.rs"]
    pub mod face_costume;
    #[path = "emoji_subgroup_face_glasses.rs"]
    pub mod face_glasses;
    #[path = "emoji_subgroup_face_hand.rs"]
    pub mod face_hand;
    #[path = "emoji_subgroup_face_hat.rs"]
    pub mod face_hat;
    #[path = "emoji_subgroup_face_negative.rs"]
    pub mod face_negative;
    #[path = "emoji_subgroup_face_neutral_skeptical.rs"]
    pub mod face_neutral_skeptical;
    #[path = "emoji_subgroup_face_sleepy.rs"]
    pub mod face_sleepy;
    #[path = "emoji_subgroup_face_smiling.rs"]
    pub mod face_smiling;
    #[path = "emoji_subgroup_face_tongue.rs"]
    pub mod face_tongue;
    #[path = "emoji_subgroup_face_unwell.rs"]
    pub mod face_unwell;
    #[path = "emoji_subgroup_monkey_face.rs"]
    pub mod monkey_face;
}
pub mod people_and_body {
    #[path = "emoji_subgroup_body_parts.rs"]
    pub mod body_parts;
    #[path = "emoji_subgroup_family.rs"]
    pub mod family;
    #[path = "emoji_subgroup_hand_fingers_closed.rs"]
    pub mod hand_fingers_closed;
    #[path = "emoji_subgroup_hand_fingers_open.rs"]
    pub mod hand_fingers_open;
    #[path = "emoji_subgroup_hand_fingers_partial.rs"]
    pub mod hand_fingers_partial;
    #[path = "emoji_subgroup_hand_prop.rs"]
    pub mod hand_prop;
    #[path = "emoji_subgroup_hand_single_finger.rs"]
    pub mod hand_single_finger;
    #[path = "emoji_subgroup_hands.rs"]
    pub mod hands;
    #[path = "emoji_subgroup_person.rs"]
    pub mod person;
    #[path = "emoji_subgroup_person_activity.rs"]
    pub mod person_activity;
    #[path = "emoji_subgroup_person_fantasy.rs"]
    pub mod person_fantasy;
    #[path = "emoji_subgroup_person_gesture.rs"]
    pub mod person_gesture;
    #[path = "emoji_subgroup_person_resting.rs"]
    pub mod person_resting;
    #[path = "emoji_subgroup_person_role.rs"]
    pub mod person_role;
    #[path = "emoji_subgroup_person_sport.rs"]
    pub mod person_sport;
    #[path = "emoji_subgroup_person_symbol.rs"]
    pub mod person_symbol;
}
pub mod component {
    #[path = "emoji_subgroup_hair_style.rs"]
    pub mod hair_style;
    #[path = "emoji_subgroup_skin_tone.rs"]
    pub mod skin_tone;
}
pub mod animals_and_nature {
    #[path = "emoji_subgroup_animal_amphibian.rs"]
    pub mod animal_amphibian;
    #[path = "emoji_subgroup_animal_bird.rs"]
    pub mod animal_bird;
    #[path = "emoji_subgroup_animal_bug.rs"]
    pub mod animal_bug;
    #[path = "emoji_subgroup_animal_mammal.rs"]
    pub mod animal_mammal;
    #[path = "emoji_subgroup_animal_marine.rs"]
    pub mod animal_marine;
    #[path = "emoji_subgroup_animal_reptile.rs"]
    pub mod animal_reptile;
    #[path = "emoji_subgroup_plant_flower.rs"]
    pub mod plant_flower;
    #[path = "emoji_subgroup_plant_other.rs"]
    pub mod plant_other;
}
pub mod food_and_drink {
    #[path = "emoji_subgroup_dishware.rs"]
    pub mod dishware;
    #[path = "emoji_subgroup_drink.rs"]
    pub mod drink;
    #[path = "emoji_subgroup_food_asian.rs"]
    pub mod food_asian;
    #[path = "emoji_subgroup_food_fruit.rs"]
    pub mod food_fruit;
    #[path = "emoji_subgroup_food_marine.rs"]
    pub mod food_marine;
    #[path = "emoji_subgroup_food_prepared.rs"]
    pub mod food_prepared;
    #[path = "emoji_subgroup_food_sweet.rs"]
    pub mod food_sweet;
    #[path = "emoji_subgroup_food_vegetable.rs"]
    pub mod food_vegetable;
}
pub mod travel_and_places {
    #[path = "emoji_subgroup_hotel.rs"]
    pub mod hotel;
    #[path = "emoji_subgroup_place_building.rs"]
    pub mod place_building;
    #[path = "emoji_subgroup_place_geographic.rs"]
    pub mod place_geographic;
    #[path = "emoji_subgroup_place_map.rs"]
    pub mod place_map;
    #[path = "emoji_subgroup_place_other.rs"]
    pub mod place_other;
    #[path = "emoji_subgroup_place_religious.rs"]
    pub mod place_religious;
    #[path = "emoji_subgroup_sky_and_weather.rs"]
    pub mod sky_and_weather;
    #[path = "emoji_subgroup_time.rs"]
    pub mod time;
    #[path = "emoji_subgroup_transport_air.rs"]
    pub mod transport_air;
    #[path = "emoji_subgroup_transport_ground.rs"]
    pub mod transport_ground;
    #[path = "emoji_subgroup_transport_water.rs"]
    pub mod transport_water;
}
pub mod activities {
    #[path = "emoji_subgroup_arts_and_crafts.rs"]
    pub mod arts_and_crafts;
    #[path = "emoji_subgroup_award_medal.rs"]
    pub mod award_medal;
    #[path = "emoji_subgroup_event.rs"]
    pub mod event;
    #[path = "emoji_subgroup_game.rs"]
    pub mod game;
    #[path = "emoji_subgroup_sport.rs"]
    pub mod sport;
}
pub mod objects {
    #[path = "emoji_subgroup_book_paper.rs"]
    pub mod book_paper;
    #[path = "emoji_subgroup_clothing.rs"]
    pub mod clothing;
    #[path = "emoji_subgroup_computer.rs"]
    pub mod computer;
    #[path = "emoji_subgroup_household.rs"]
    pub mod household;
    #[path = "emoji_subgroup_light_and_video.rs"]
    pub mod light_and_video;
    #[path = "emoji_subgroup_lock.rs"]
    pub mod lock;
    #[path = "emoji_subgroup_mail.rs"]
    pub mod mail;
    #[path = "emoji_subgroup_medical.rs"]
    pub mod medical;
    #[path = "emoji_subgroup_money.rs"]
    pub mod money;
    #[path = "emoji_subgroup_music.rs"]
    pub mod music;
    #[path = "emoji_subgroup_musical_instrument.rs"]
    pub mod musical_instrument;
    #[path = "emoji_subgroup_office.rs"]
    pub mod office;
    #[path = "emoji_subgroup_other_object.rs"]
    pub mod other_object;
    #[path = "emoji_subgroup_phone.rs"]
    pub mod phone;
    #[path = "emoji_subgroup_science.rs"]
    pub mod science;
    #[path = "emoji_subgroup_sound.rs"]
    pub mod sound;
    #[path = "emoji_subgroup_tool.rs"]
    pub mod tool;
    #[path = "emoji_subgroup_writing.rs"]
    pub mod writing;
}
pub mod symbols {
    #[path = "emoji_subgroup_alphanum.rs"]
    pub mod alphanum;
    #[path = "emoji_subgroup_arrow.rs"]
    pub mod arrow;
    #[path = "emoji_subgroup_av_symbol.rs"]
    pub mod av_symbol;
    #[path = "emoji_subgroup_currency.rs"]
    pub mod currency;
    #[path = "emoji_subgroup_gender.rs"]
    pub mod gender;
    #[path = "emoji_subgroup_geometric.rs"]
    pub mod geometric;
    #[path = "emoji_subgroup_keycap.rs"]
    pub mod keycap;
    #[path = "emoji_subgroup_math.rs"]
    pub mod math;
    #[path = "emoji_subgroup_other_symbol.rs"]
    pub mod other_symbol;
    #[path = "emoji_subgroup_punctuation.rs"]
    pub mod punctuation;
    #[path = "emoji_subgroup_religion.rs"]
    pub mod religion;
    #[path = "emoji_subgroup_transport_sign.rs"]
    pub mod transport_sign;
    #[path = "emoji_subgroup_warning.rs"]
    pub mod warning;
    #[path = "emoji_subgroup_zodiac.rs"]
    pub mod zodiac;
}
pub mod flags {
    #[path = "emoji_subgroup_country_flag.rs"]
    pub mod country_flag;
    #[path = "emoji_subgroup_flag.rs"]
    pub mod flag;
    #[path = "emoji_subgroup_subdivision_flag.rs"]
    pub mod subdivision_flag;
}