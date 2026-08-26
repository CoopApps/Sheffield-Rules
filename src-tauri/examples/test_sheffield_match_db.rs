/// Test Sheffield 1867 Match Simulator with Real Database Players
/// Run with: cargo run --example test_sheffield_match_db

use saturday_at_three::{database, commands::PlayerDetail};
use saturday_at_three::match_engine::sheffield_1867::{SheffieldMatchSimulator, RealPlayerData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║     SHEFFIELD RULES MATCH SIMULATION - DATABASE TEST      ║");
    println!("║              Sheffield FC vs Hallam FC (1867)              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Club IDs from Sheffield1867.db
    let sheffield_fc_id = "sheffield-fc";
    let hallam_fc_id = "hallam-fc";

    println!("📊 Loading squads from database...\n");

    // Load Sheffield FC squad
    let sheffield_squad = match database::get_club_squad(sheffield_fc_id).await {
        Ok(players) => {
            println!("✅ Sheffield FC: {} players loaded", players.len());
            players
        }
        Err(e) => {
            eprintln!("❌ Failed to load Sheffield FC squad: {}", e);
            println!("   Using generic team instead");
            Vec::new()
        }
    };

    // Load Hallam FC squad
    let hallam_squad = match database::get_club_squad(hallam_fc_id).await {
        Ok(players) => {
            println!("✅ Hallam FC: {} players loaded", players.len());
            players
        }
        Err(e) => {
            eprintln!("❌ Failed to load Hallam FC squad: {}", e);
            println!("   Using generic team instead");
            Vec::new()
        }
    };

    println!("\n═══════════════════════════════════════════════════════════\n");

    // Convert to RealPlayerData
    let convert_squad = |squad: Vec<PlayerDetail>| -> Vec<RealPlayerData> {
        squad.into_iter().map(|p| RealPlayerData {
            id: p.id.clone(),
            name: p.name.clone(),
            position: p.position.clone(),
            age: p.age,
            pace: p.pace,
            acceleration: p.acceleration,
            strength: p.strength,
            stamina: p.stamina,
            agility: p.agility,
            passing: p.passing,
            dribbling: p.dribbling,
            first_touch: p.first_touch,
            heading: p.heading,
            finishing: p.finishing,
            tackling: p.tackling,
            composure: p.concentration,
            vision: p.vision,
            decisions: p.decision_making,
            positioning: p.positioning,
            teamwork: p.teamwork,
            work_rate: p.work_rate,
        }).collect()
    };

    let sheffield_real = convert_squad(sheffield_squad);
    let hallam_real = convert_squad(hallam_squad);

    // Create simulator with real player data
    let mut simulator = if sheffield_real.is_empty() || hallam_real.is_empty() {
        println!("⚠️  Using generic team strengths (database players not available)\n");
        SheffieldMatchSimulator::new(
            "Sheffield FC".to_string(),
            "Hallam FC".to_string(),
            0.65,
            0.60,
        )
    } else {
        println!("✅ Using real player attributes from database\n");
        SheffieldMatchSimulator::new_with_squads(
            "Sheffield FC".to_string(),
            "Hallam FC".to_string(),
            sheffield_real,
            hallam_real,
        )
    };

    println!("🏟️  Kick-off imminent!\n");
    println!("═══════════════════════════════════════════════════════════\n");

    // Simulate the match
    let result = simulator.simulate_match_fast();

    // Display key events
    println!("\n📋 KEY MATCH EVENTS:\n");
    for event in &result.events {
        let icon = match event.event_type {
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::KickOff => "⚽",
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::Goal => "🎯",
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::Rouge => "🟠",
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::InjuryMinor => "🤕",
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::Injury => "⚕️",
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::InjuryMajor => "🚑",
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::HalfTime => "⏸️",
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::FullTime => "🏁",
            _ => continue,
        };

        println!("{} [{}'] {}", icon, event.minute, event.description);

        // Show score for goals/rouges
        if matches!(event.event_type,
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::Goal |
            saturday_at_three::match_engine::sheffield_1867::match_simulator::EventType::Rouge) {
            println!("   Score: {}G {}R - {}G {}R",
                event.home_score, event.home_rouges,
                event.away_score, event.away_rouges);
        }
    }

    // Final result
    println!("\n═══════════════════════════════════════════════════════════");
    println!("\n🏁 FULL TIME RESULT:");
    println!("   Sheffield FC - {} goals, {} rouges", result.home_goals, result.home_rouges);
    println!("   Hallam FC    - {} goals, {} rouges", result.away_goals, result.away_rouges);

    let winner_text = match result.match_winner {
        saturday_at_three::match_engine::sheffield_1867::match_simulator::MatchWinner::HomeTeam => "🏆 Sheffield FC wins by goals!",
        saturday_at_three::match_engine::sheffield_1867::match_simulator::MatchWinner::AwayTeam => "🏆 Hallam FC wins by goals!",
        saturday_at_three::match_engine::sheffield_1867::match_simulator::MatchWinner::HomeTeamOnRouges => "🏆 Sheffield FC wins on rouges!",
        saturday_at_three::match_engine::sheffield_1867::match_simulator::MatchWinner::AwayTeamOnRouges => "🏆 Hallam FC wins on rouges!",
        saturday_at_three::match_engine::sheffield_1867::match_simulator::MatchWinner::Draw => "🤝 Match drawn!",
    };
    println!("\n{}", winner_text);

    // Match statistics
    println!("\n═══════════════════════════════════════════════════════════");
    println!("\n📊 MATCH STATISTICS:\n");
    println!("   Possession:    Sheffield {:.1}% - {:.1}% Hallam",
        result.possession_home * 100.0, result.possession_away * 100.0);
    println!("   Total Shots:   Sheffield {} - {} Hallam",
        result.total_shots_home, result.total_shots_away);
    println!("   Total Events:  {}", result.events.len());

    // Injury report
    if !result.injuries.is_empty() {
        println!("\n🚑 INJURY REPORT:");
        for injury in &result.injuries {
            println!("   [{}'] {}", injury.minute, injury.description);
            if !injury.can_continue {
                println!("       ⚠️  Player unable to continue!");
            }
        }
    }

    // Weather & pitch
    println!("\n🌤️  CONDITIONS:");
    println!("   Weather: {:?}", result.weather);
    println!("   Pitch:   {:?}", result.pitch);

    // Top performers
    println!("\n⭐ TOP PERFORMERS:\n");

    let mut all_players = result.home_player_stats.clone();
    all_players.extend(result.away_player_stats.clone());

    // Sort by goals
    all_players.sort_by(|a, b| b.goals_scored.cmp(&a.goals_scored));
    if let Some(top_scorer) = all_players.first() {
        if top_scorer.goals_scored > 0 {
            println!("   ⚽ Top Scorer: {} ({} goals)", top_scorer.player_name, top_scorer.goals_scored);
        }
    }

    // Sort by rouges
    all_players.sort_by(|a, b| b.rouges_scored.cmp(&a.rouges_scored));
    if let Some(top_rouge) = all_players.first() {
        if top_rouge.rouges_scored > 0 {
            println!("   🟠 Most Rouges: {} ({} rouges)", top_rouge.player_name, top_rouge.rouges_scored);
        }
    }

    // Sort by tackles
    all_players.sort_by(|a, b| b.tackles_won.cmp(&a.tackles_won));
    if let Some(top_tackler) = all_players.first() {
        if top_tackler.tackles_won > 0 {
            println!("   🛡️  Best Defender: {} ({} tackles won)", top_tackler.player_name, top_tackler.tackles_won);
        }
    }

    println!("\n═══════════════════════════════════════════════════════════\n");

    Ok(())
}
