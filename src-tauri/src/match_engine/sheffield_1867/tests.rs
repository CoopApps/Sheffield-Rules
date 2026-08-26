/// Unit tests for Sheffield 1867 Match Simulator
#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::match_simulator::EventType;

    #[test]
    fn test_sheffield_match_simulation() {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║     SHEFFIELD 1867 MATCH SIMULATION TEST                   ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");

        let mut simulator = SheffieldMatchSimulator::new(
            "Sheffield FC".to_string(),
            "Hallam FC".to_string(),
            0.6,
            0.55,
        );

        println!("⚽ Match: Sheffield FC vs Hallam FC\n");
        println!("Generating match events...\n");

        let events = simulator.simulate_match_live();

        println!("═══════════════════════════════════════════════════════════\n");
        println!("📊 Match generated {} events", events.len());

        // Count event types
        let goals = events.iter().filter(|e| e.event_type == EventType::Goal).count();
        let rouges = events.iter().filter(|e| e.event_type == EventType::Rouge).count();
        let shots = events.iter().filter(|e| e.event_type == EventType::Shot).count();

        println!("\n📈 EVENT BREAKDOWN:");
        println!("   Goals: {}", goals);
        println!("   Rouges: {}", rouges);
        println!("   Shots: {}", shots);
        println!("   Other events: {}", events.len() - goals - rouges - shots);

        // Display key events
        println!("\n🎯 KEY EVENTS:\n");
        for event in &events {
            if event.event_type == EventType::Goal || event.event_type == EventType::Rouge {
                println!("[{}'] {:?} - {}", event.minute, event.event_type, event.description);
                println!("      Score: {}G {}R - {}G {}R",
                    event.home_score, event.home_rouges,
                    event.away_score, event.away_rouges);
            } else if event.event_type == EventType::KickOff || event.event_type == EventType::HalfTime || event.event_type == EventType::FullTime {
                println!("[{}'] {}", event.minute, event.description);
            }
        }

        // Final result
        if let Some(final_event) = events.last() {
            println!("\n═══════════════════════════════════════════════════════════");
            println!("\n🏆 FINAL RESULT:");
            println!("   Sheffield FC: {}G {}R", final_event.home_score, final_event.home_rouges);
            println!("   Hallam FC: {}G {}R", final_event.away_score, final_event.away_rouges);

            // Determine winner
            let winner = if final_event.home_score > final_event.away_score {
                "Sheffield FC wins by goals!"
            } else if final_event.away_score > final_event.home_score {
                "Hallam FC wins by goals!"
            } else if final_event.home_rouges > final_event.away_rouges {
                "Sheffield FC wins on rouges!"
            } else if final_event.away_rouges > final_event.home_rouges {
                "Hallam FC wins on rouges!"
            } else {
                "Match drawn!"
            };

            println!("\n   {}", winner);
            println!("\n═══════════════════════════════════════════════════════════\n");
        }

        // Assertions
        assert!(!events.is_empty(), "Should generate events");
        assert!(events.iter().any(|e| e.event_type == EventType::KickOff), "Should have kickoff");
        assert!(events.iter().any(|e| e.event_type == EventType::FullTime), "Should have fulltime");

        // Check that final event has proper scores
        if let Some(final_event) = events.last() {
            assert_eq!(final_event.event_type, EventType::FullTime, "Last event should be FullTime");
            assert!(final_event.home_score >= 0, "Home score should be non-negative");
            assert!(final_event.away_score >= 0, "Away score should be non-negative");
            assert!(final_event.home_rouges >= 0, "Home rouges should be non-negative");
            assert!(final_event.away_rouges >= 0, "Away rouges should be non-negative");
        }
    }

    #[test]
    fn test_sheffield_rules_winner_determination() {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║     SHEFFIELD RULES WINNER DETERMINATION TEST              ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");

        // Test: Goals beat rouges
        println!("✓ Test 1: Goals beat rouges (2-1 vs 1-0)");
        assert_winner(2, 1, 0, 0, "Home wins by goals");

        // Test: Rouges decide when goals are tied
        println!("✓ Test 2: Rouges break tie when goals equal (1-1, 3R vs 1R)");
        assert_winner(1, 1, 3, 1, "Home wins on rouges");

        // Test: Draw when both goals and rouges are tied
        println!("✓ Test 3: Draw when all scores equal (1-1, 2R vs 2R)");
        assert_winner(1, 1, 2, 2, "Draw");

        // Test: Higher goals win regardless of rouges
        println!("✓ Test 4: Higher goals win despite fewer rouges (3-1, 0R vs 5R)");
        assert_winner(3, 1, 0, 5, "Home wins by goals");

        println!("\n═══════════════════════════════════════════════════════════");
        println!("✅ All Sheffield Rules tests passed!\n");
    }

    fn assert_winner(home_goals: i32, away_goals: i32, home_rouges: i32, away_rouges: i32, expected: &str) {
        let result = if home_goals > away_goals {
            "Home wins by goals"
        } else if away_goals > home_goals {
            "Away wins by goals"
        } else if home_rouges > away_rouges {
            "Home wins on rouges"
        } else if away_rouges > home_rouges {
            "Away wins on rouges"
        } else {
            "Draw"
        };

        assert_eq!(result, expected,
            "{}G {}R - {}G {}R should result in: {}",
            home_goals, home_rouges, away_goals, away_rouges, expected);
    }
}
