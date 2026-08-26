// Placeholder for match simulation
// Will integrate with Football Man simulator later

pub fn simulate_match_basic(home_strength: f32, away_strength: f32) -> (u8, u8) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Simple Poisson-like simulation
    let home_expected = home_strength + 0.1; // 10% home advantage
    let away_expected = away_strength;

    let home_score = (rng.gen::<f32>() * home_expected * 2.0).floor() as u8;
    let away_score = (rng.gen::<f32>() * away_expected * 2.0).floor() as u8;

    (home_score, away_score)
}
