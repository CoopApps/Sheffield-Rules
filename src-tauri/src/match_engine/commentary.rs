/// Commentary generation for match events
/// Provides varied, contextual descriptions for match events

use crate::match_engine::possession_engine::FieldZone;
use rand::Rng;

pub struct CommentaryLibrary;

impl CommentaryLibrary {
    /// Generate commentary for a successful pass
    pub fn pass_commentary(rng: &mut impl Rng, passes_in_sequence: usize, zone: FieldZone) -> String {
        let zone_desc = match zone {
            FieldZone::DefensiveThird => &["in their own half", "deep in defense", "near their goal"][rng.gen_range(0..3)],
            FieldZone::Midfield => &["in midfield", "at the center", "in the middle third"][rng.gen_range(0..3)],
            FieldZone::AttackingThird => &["in the attacking third", "near the opposition goal", "high up the pitch"][rng.gen_range(0..3)],
        };

        if passes_in_sequence > 5 {
            let phrases = [
                format!("Excellent passing movement {}!", zone_desc),
                format!("Building from the back {}, {} passes now", zone_desc, passes_in_sequence),
                format!("Patient buildup {}!", zone_desc),
                format!("They're keeping possession well {}", zone_desc),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        } else {
            let phrases = [
                format!("Ball moved forward {}", zone_desc),
                format!("Pass completed {}", zone_desc),
                format!("Good interplay {}", zone_desc),
                format!("Keeping the ball {}", zone_desc),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
    }

    /// Generate commentary for a turnover
    pub fn turnover_commentary(rng: &mut impl Rng, zone: FieldZone) -> String {
        let location = match zone {
            FieldZone::DefensiveThird => "in a dangerous area",
            FieldZone::Midfield => "in midfield",
            FieldZone::AttackingThird => "deep in opposition territory",
        };

        let phrases = [
            format!("Possession lost {}!", location),
            format!("Turned over {}!", location),
            format!("They lose the ball {}!", location),
            format!("Dispossessed {}!", location),
            format!("Poor touch and it's given away {}!", location),
            format!("Intercepted {}!", location),
        ];
        phrases[rng.gen_range(0..phrases.len())].clone()
    }

    /// Generate commentary for a shot
    pub fn shot_commentary(rng: &mut impl Rng, on_target: bool) -> String {
        if on_target {
            let phrases = [
                "Shot! On target!",
                "He strikes! Heading for goal!",
                "Opportunity! Shot on goal!",
                "Here's the chance! He shoots!",
                "Dangerous shot!",
                "Fires it goalward!",
                "Takes aim and shoots!",
            ];
            phrases[rng.gen_range(0..phrases.len())].to_string()
        } else {
            let phrases = [
                "Shot goes wide!",
                "Blazes it over the bar!",
                "Off target!",
                "Misses the target!",
                "Wide of the mark!",
                "Couldn't direct it on goal!",
                "Shot sails high and wide!",
            ];
            phrases[rng.gen_range(0..phrases.len())].to_string()
        }
    }

    /// Generate commentary for a goal
    pub fn goal_commentary(rng: &mut impl Rng, scorer: &str) -> String {
        let phrases = [
            format!("⚽ GOAL! {} finds the net!", scorer),
            format!("⚽ IT'S IN! {} scores!", scorer),
            format!("⚽ MAGNIFICENT! {} puts it away!", scorer),
            format!("⚽ What a finish by {}!", scorer),
            format!("⚽ {} breaks through and scores!", scorer),
            format!("⚽ The ball is in the back of the net! {} scores!", scorer),
            format!("⚽ SUPERB! {} makes no mistake!", scorer),
        ];
        phrases[rng.gen_range(0..phrases.len())].clone()
    }

    /// Generate commentary for a save
    pub fn save_commentary(rng: &mut impl Rng, keeper: &str) -> String {
        let phrases = [
            format!("Brilliant save by {}!", keeper),
            format!("{} denies them!", keeper),
            format!("What a stop from {}!", keeper),
            format!("{} pulls off a great save!", keeper),
            format!("Saved! {} to the rescue!", keeper),
            format!("{} keeps it out!", keeper),
            format!("Tremendous goalkeeping from {}!", keeper),
        ];
        phrases[rng.gen_range(0..phrases.len())].clone()
    }

    /// Generate commentary for a rouge (Sheffield Rules specific)
    pub fn rouge_commentary(rng: &mut impl Rng) -> String {
        let phrases = [
            "Rouge awarded! Ball placed behind the goal",
            "It's a rouge! Forced behind",
            "Rouge! Defensive placement",
            "Behind the goal for a rouge",
            "Rouge scored! Ball went behind",
        ];
        phrases[rng.gen_range(0..phrases.len())].to_string()
    }

    /// Generate commentary for out of play
    pub fn out_of_play_commentary(rng: &mut impl Rng) -> String {
        let phrases = [
            "Ball out of play",
            "Gone out for a throw-in",
            "Out of bounds",
            "Play stopped, ball out",
            "Off the pitch",
            "Into touch",
        ];
        phrases[rng.gen_range(0..phrases.len())].to_string()
    }

    /// Generate commentary for dangerous attack
    pub fn attack_commentary(rng: &mut impl Rng) -> String {
        let phrases = [
            "Pressing forward now!",
            "On the attack!",
            "Dangerous move developing!",
            "They're pushing up the field!",
            "Building an attack here!",
            "Looking threatening!",
            "Advancing with purpose!",
        ];
        phrases[rng.gen_range(0..phrases.len())].to_string()
    }

    /// Generate commentary for defensive action
    pub fn defense_commentary(rng: &mut impl Rng) -> String {
        let phrases = [
            "Good defensive work!",
            "Cleared away!",
            "Danger averted!",
            "Defending stoutly!",
            "Strong tackle!",
            "Well defended!",
            "Solid at the back!",
        ];
        phrases[rng.gen_range(0..phrases.len())].to_string()
    }

    /// Generate commentary for a foul
    pub fn foul_commentary(rng: &mut impl Rng, zone: FieldZone) -> String {
        let location = match zone {
            FieldZone::DefensiveThird => "in a dangerous position",
            FieldZone::Midfield => "in the middle of the park",
            FieldZone::AttackingThird => "near the goal",
        };

        let phrases = [
            format!("Foul {}!", location),
            format!("Free kick awarded {}!", location),
            format!("The referee blows for a foul {}!", location),
            format!("Pulled back {}! Free kick", location),
        ];
        phrases[rng.gen_range(0..phrases.len())].clone()
    }

    /// Generate corner kick commentary
    pub fn corner_commentary(rng: &mut impl Rng) -> String {
        let phrases = [
            "Corner kick awarded!",
            "It's a corner!",
            "Corner to come",
            "They've won a corner",
            "Corner kick - dangerous opportunity!",
        ];
        phrases[rng.gen_range(0..phrases.len())].to_string()
    }

    /// Generate throw-in commentary
    pub fn throw_in_commentary(rng: &mut impl Rng) -> String {
        let phrases = [
            "Throw-in",
            "Out for a throw",
            "Throw-in to be taken",
            "Ball thrown in",
        ];
        phrases[rng.gen_range(0..phrases.len())].to_string()
    }
}
