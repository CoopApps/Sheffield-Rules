/// Test Sheffield 1867 Match Simulator
/// Run with: cargo run --example test_sheffield_match

use saturday_at_three::match_engine::sheffield_1867::SheffieldMatchSimulator;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║        SHEFFIELD RULES MATCH SIMULATION TEST               ║");
    println!("║                    1867 Edition                            ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Create a match between Sheffield FC and Hallam FC
    let home_team = "Sheffield FC".to_string();
    let away_team = "Hallam FC".to_string();
    let home_strength = 0.6;
    let away_strength = 0.55;

    println!("🏟️  Match: {} vs {}", home_team, away_team);
    println!("⚖️  Team Strengths: {:.1}% vs {:.1}%", home_strength * 100.0, away_strength * 100.0);
    println!("\n═══════════════════════════════════════════════════════════\n");

    // Initialize the simulator
    let mut simulator = SheffieldMatchSimulator::new(
        home_team.clone(),
        away_team.clone(),
        home_strength,
        away_strength,
    );

    // Generate match events
    let events = simulator.simulate_match_live();

    // Display all events with Victorian styling
    for event in &events {
        let icon = match event.event_type.as_str() {
            "KickOff" => "⚽",
            "Goal" => "🎯",
            "Rouge" => "🟠",
            "Shot" => "↗️",
            "Pass" => "→",
            "Tackle" => "🛡️",
            "HalfTime" => "⏸️",
            "FullTime" => "🏁",
            _ => "•",
        };

        // Color-code important events
        if event.event_type == "Goal" {
            println!("\n{} [{}'] {} - GOAL!", icon, event.minute, event.event_type);
            println!("   {}", event.description);
            println!("   Score: {} {} - {} {}",
                event.home_score, event.home_rouges,
                event.away_score, event.away_rouges);
        } else if event.event_type == "Rouge" {
            println!("\n{} [{}'] {} - ROUGE!", icon, event.minute, event.event_type);
            println!("   {}", event.description);
            println!("   Score: {}G {}R - {}G {}R",
                event.home_score, event.home_rouges,
                event.away_score, event.away_rouges);
        } else if event.event_type == "KickOff" || event.event_type == "HalfTime" || event.event_type == "FullTime" {
            println!("\n{} [{}'] {}", icon, event.minute, event.event_type);
            println!("   {}", event.description);
        } else {
            println!("{} [{}'] {}", icon, event.minute, event.description);
        }
    }

    // Final result
    if let Some(final_event) = events.last() {
        println!("\n═══════════════════════════════════════════════════════════");
        println!("\n📊 FINAL RESULT:");
        println!("   {} - {}G {}R", home_team, final_event.home_score, final_event.home_rouges);
        println!("   {} - {}G {}R", away_team, final_event.away_score, final_event.away_rouges);

        // Determine winner using Sheffield Rules
        let winner = if final_event.home_score > final_event.away_score {
            format!("{} wins by goals!", home_team)
        } else if final_event.away_score > final_event.home_score {
            format!("{} wins by goals!", away_team)
        } else if final_event.home_rouges > final_event.away_rouges {
            format!("{} wins on rouges!", home_team)
        } else if final_event.away_rouges > final_event.home_rouges {
            format!("{} wins on rouges!", away_team)
        } else {
            "Match drawn!".to_string()
        };

        println!("\n🏆 {}", winner);
        println!("\n═══════════════════════════════════════════════════════════\n");
    }

    // Statistics
    let goals = events.iter().filter(|e| e.event_type == "Goal").count();
    let rouges = events.iter().filter(|e| e.event_type == "Rouge").count();
    let shots = events.iter().filter(|e| e.event_type == "Shot").count();

    println!("📈 MATCH STATISTICS:");
    println!("   Total Events: {}", events.len());
    println!("   Goals Scored: {}", goals);
    println!("   Rouges Scored: {}", rouges);
    println!("   Shots Attempted: {}", shots);
    println!();
}
