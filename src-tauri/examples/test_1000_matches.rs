/// Run 1000 Sheffield FC vs Hallam FC matches and analyze statistics
/// Run with: cargo run --example test_1000_matches

use saturday_at_three::{database, commands::PlayerDetail};
use saturday_at_three::match_engine::sheffield_1867::{SheffieldMatchSimulator, RealPlayerData};
use saturday_at_three::match_engine::sheffield_1867::match_simulator::MatchWinner;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║         1000 MATCH SIMULATION - STATISTICAL ANALYSIS      ║");
    println!("║              Sheffield FC vs Hallam FC (1867)              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Load squads
    let sheffield_fc_id = "sheffield-fc";
    let hallam_fc_id = "hallam-fc";

    println!("📊 Loading squads from database...\n");

    let sheffield_squad = database::get_club_squad(sheffield_fc_id).await
        .unwrap_or_else(|_| Vec::new());
    let hallam_squad = database::get_club_squad(hallam_fc_id).await
        .unwrap_or_else(|_| Vec::new());

    if sheffield_squad.is_empty() || hallam_squad.is_empty() {
        println!("❌ Failed to load squads. Exiting.");
        return Ok(());
    }

    println!("✅ Sheffield FC: {} players", sheffield_squad.len());
    println!("✅ Hallam FC: {} players\n", hallam_squad.len());

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

    // Statistics tracking
    let mut sheffield_wins = 0;
    let mut hallam_wins = 0;
    let mut sheffield_wins_on_rouges = 0;
    let mut hallam_wins_on_rouges = 0;
    let mut draws = 0;
    let mut total_goals = 0;
    let mut total_rouges = 0;
    let mut total_injuries = 0;
    let mut severe_injuries = 0;
    let mut sheffield_goals = 0;
    let mut hallam_goals = 0;
    let mut sheffield_rouges = 0;
    let mut hallam_rouges = 0;
    let mut goal_scorers: HashMap<String, i32> = HashMap::new();
    let mut rouge_scorers: HashMap<String, i32> = HashMap::new();

    let matches_to_run = 1000;

    println!("⚽ Simulating {} matches...\n", matches_to_run);

    let start_time = std::time::Instant::now();

    for i in 0..matches_to_run {
        if (i + 1) % 100 == 0 {
            print!("\rProgress: {}/{} matches", i + 1, matches_to_run);
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }

        let mut simulator = SheffieldMatchSimulator::new_with_squads(
            "Sheffield FC".to_string(),
            "Hallam FC".to_string(),
            sheffield_real.clone(),
            hallam_real.clone(),
        );

        let result = simulator.simulate_match_fast();

        // Count results
        match result.match_winner {
            MatchWinner::HomeTeam => sheffield_wins += 1,
            MatchWinner::AwayTeam => hallam_wins += 1,
            MatchWinner::HomeTeamOnRouges => {
                sheffield_wins += 1;
                sheffield_wins_on_rouges += 1;
            }
            MatchWinner::AwayTeamOnRouges => {
                hallam_wins += 1;
                hallam_wins_on_rouges += 1;
            }
            MatchWinner::Draw => draws += 1,
        }

        // Track goals and rouges
        sheffield_goals += result.home_goals;
        hallam_goals += result.away_goals;
        sheffield_rouges += result.home_rouges;
        hallam_rouges += result.away_rouges;
        total_goals += result.home_goals + result.away_goals;
        total_rouges += result.home_rouges + result.away_rouges;

        // Track scorers
        for player in &result.home_player_stats {
            if player.goals_scored > 0 {
                *goal_scorers.entry(player.player_name.clone()).or_insert(0) += player.goals_scored;
            }
            if player.rouges_scored > 0 {
                *rouge_scorers.entry(player.player_name.clone()).or_insert(0) += player.rouges_scored;
            }
        }
        for player in &result.away_player_stats {
            if player.goals_scored > 0 {
                *goal_scorers.entry(player.player_name.clone()).or_insert(0) += player.goals_scored;
            }
            if player.rouges_scored > 0 {
                *rouge_scorers.entry(player.player_name.clone()).or_insert(0) += player.rouges_scored;
            }
        }

        // Track injuries
        total_injuries += result.injuries.len();
        severe_injuries += result.injuries.iter().filter(|inj| !inj.can_continue).count();
    }

    let duration = start_time.elapsed();
    println!("\r✅ Completed {} matches in {:.2}s\n", matches_to_run, duration.as_secs_f64());

    // Print statistics
    println!("═══════════════════════════════════════════════════════════");
    println!("\n📊 OVERALL STATISTICS\n");
    println!("Matches Simulated: {}", matches_to_run);
    println!("Average Duration:  {:.2}ms per match", duration.as_millis() as f64 / matches_to_run as f64);

    println!("\n🏆 MATCH OUTCOMES:\n");
    println!("Sheffield FC Wins:       {} ({:.1}%)", sheffield_wins, sheffield_wins as f64 / matches_to_run as f64 * 100.0);
    println!("  - Wins on Rouges:      {} ({:.1}%)", sheffield_wins_on_rouges, sheffield_wins_on_rouges as f64 / sheffield_wins.max(1) as f64 * 100.0);
    println!("Hallam FC Wins:          {} ({:.1}%)", hallam_wins, hallam_wins as f64 / matches_to_run as f64 * 100.0);
    println!("  - Wins on Rouges:      {} ({:.1}%)", hallam_wins_on_rouges, hallam_wins_on_rouges as f64 / hallam_wins.max(1) as f64 * 100.0);
    println!("Draws:                   {} ({:.1}%)", draws, draws as f64 / matches_to_run as f64 * 100.0);

    println!("\n⚽ SCORING STATISTICS:\n");
    println!("Total Goals:             {}", total_goals);
    println!("Total Rouges:            {}", total_rouges);
    println!("Average Goals/Match:     {:.2}", total_goals as f64 / matches_to_run as f64);
    println!("Average Rouges/Match:    {:.2}", total_rouges as f64 / matches_to_run as f64);

    println!("\n📈 TEAM AVERAGES:\n");
    println!("Sheffield FC:");
    println!("  Goals/Match:           {:.2}", sheffield_goals as f64 / matches_to_run as f64);
    println!("  Rouges/Match:          {:.2}", sheffield_rouges as f64 / matches_to_run as f64);
    println!("Hallam FC:");
    println!("  Goals/Match:           {:.2}", hallam_goals as f64 / matches_to_run as f64);
    println!("  Rouges/Match:          {:.2}", hallam_rouges as f64 / matches_to_run as f64);

    println!("\n🚑 INJURY STATISTICS:\n");
    println!("Total Injuries:          {}", total_injuries);
    println!("Severe Injuries:         {} ({:.1}%)", severe_injuries, severe_injuries as f64 / total_injuries.max(1) as f64 * 100.0);
    println!("Average Injuries/Match:  {:.2}", total_injuries as f64 / matches_to_run as f64);

    // Top scorers
    println!("\n⭐ TOP 10 GOAL SCORERS:\n");
    let mut scorers_vec: Vec<_> = goal_scorers.iter().collect();
    scorers_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (i, (name, goals)) in scorers_vec.iter().take(10).enumerate() {
        println!("{}. {} - {} goals", i + 1, name, goals);
    }

    // Top rouge scorers
    if total_rouges > 0 {
        println!("\n🟠 TOP 10 ROUGE SCORERS:\n");
        let mut rouge_vec: Vec<_> = rouge_scorers.iter().collect();
        rouge_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (i, (name, rouges)) in rouge_vec.iter().take(10).enumerate() {
            println!("{}. {} - {} rouges", i + 1, name, rouges);
        }
    } else {
        println!("\n🟠 NO ROUGES SCORED IN ANY MATCH!");
        println!("   This may indicate an issue with rouge probability settings.");
    }

    println!("\n═══════════════════════════════════════════════════════════\n");

    Ok(())
}
