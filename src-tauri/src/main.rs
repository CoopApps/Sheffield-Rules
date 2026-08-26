// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;
mod commands;
mod database;
mod fsim_bridge;
mod simulation;
mod sheffield_rules;
mod season_engine;
mod match_engine;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Initialize Sheffield database on app startup
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                let db_path = "D:/projects/Saturday at Three/saturday_at_three.db";
                match database::sheffield_db::create_sheffield_database(db_path).await {
                    Ok(pool) => {
                        // Initialize schema
                        if let Err(e) = database::sheffield_db::initialize_schema(&pool).await {
                            eprintln!("Failed to initialize schema: {}", e);
                        }
                        // Populate initial data if needed
                        if let Err(e) = database::populate::populate_clubs_for_year(&pool, 1857, false).await {
                            eprintln!("Failed to populate clubs: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to create database: {}", e);
                    }
                }

                // Initialize Sheffield1867.db — the master database for the database editor
                // IMPORTANT: Use get_pool_write() for schema initialization since get_pool() is read-only
                let write_pool = match database::get_pool_write().await {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Failed to get Sheffield1867 write pool: {}", e);
                        return;
                    }
                };

                // Ensure schema exists (requires write access)
                if let Err(e) = database::sheffield_db::initialize_schema(&write_pool).await {
                    eprintln!("Failed to initialize Sheffield1867 schema: {}", e);
                }

                // Populate all 186 clubs if the table is empty — never wipes existing data
                let club_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_clubs")
                    .fetch_one(&write_pool)
                    .await
                    .unwrap_or((0,));
                if club_count.0 == 0 {
                    if let Err(e) = database::populate::populate_clubs_for_year(&write_pool, 1857, true).await {
                        eprintln!("Failed to populate clubs into Sheffield1867: {}", e);
                    }
                }

                if let Err(e) = database::league_metadata::create_league_metadata_table(&write_pool).await {
                    eprintln!("Failed to create league metadata table: {}", e);
                }
                if let Err(e) = database::league_config::create_league_config_table(&write_pool).await {
                    eprintln!("Failed to create league config table: {}", e);
                }
                // Initialize default Fantasy 1867 league
                let _ = database::league_metadata::initialize_default_league(&write_pool).await.map_err(|e| {
                    eprintln!("Failed to initialize default league: {}", e);
                });

                // Create indexes for sheffield_people to speed up searches
                if let Err(e) = database::whites_matcher::create_people_indexes(&write_pool).await {
                    eprintln!("Failed to create people indexes: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::new_game,
            commands::load_game,
            commands::save_game,
            commands::get_current_game,
            commands::advance_gameweek,
            commands::advance_day,
            commands::complete_user_event,
            commands::get_upcoming_fixtures,
            commands::get_standings,
            commands::arrange_training,
            commands::muster_squad,
            commands::committee_status,
            commands::club_identity,
            commands::sponsored_cups,
            commands::coop_status,
            commands::coop_buy,
            commands::get_squad,
            commands::get_all_clubs,
            commands::get_club_squad,
            commands::get_club_info,
            commands::get_reserve_team_id,
            commands::get_sheffield_game_modes,
            commands::new_sheffield_game,
            commands::get_sheffield_ruleset_for_season,
            commands::get_all_sheffield_years,
            commands::get_sheffield_year_summary,
            commands::get_sheffield_clubs,
            commands::get_sheffield_clubs_from_db,
            commands::get_sheffield_clubs_for_mode,
            commands::initialize_sheffield_game,
            commands::get_division_clubs,
            commands::get_division_standings,
            commands::get_club_league_division,
            commands::get_club_division_info,
            commands::process_season_promotions_relegations,
            commands::db_get_all_clubs,
            commands::db_get_club_players,
            commands::db_get_squad_simple,
            commands::db_bulk_add_players,
            commands::db_bulk_add_players_with_ages,
            commands::db_update_club_location,
            commands::db_update_club_name,
            commands::db_copy_parent_data_to_reserve,
            commands::db_create_club,
            commands::db_get_unassigned_clubs,
            commands::db_get_all_divisions,
            commands::db_update_club_division,
            commands::db_import_league_structure,
            commands::db_update_player_stats,
            commands::transfer_player,
            commands::db_create_annual_cups,
            commands::db_create_custom_cup,
            commands::db_update_cup_competition,
            commands::db_get_eligible_clubs,
            commands::db_generate_cup_draw,
            commands::db_get_cup_bracket,
            commands::db_get_competitions_for_season,
            commands::db_delete_competitions_for_season,
            commands::db_run_schema_migrations,
            commands::db_get_all_players,
            // Genealogy Matching Test
            commands::db_test_genealogy_matching,
            // Pagination and Statistics Commands
            commands::db_get_players_paginated,
            commands::db_get_age_statistics,
            commands::db_get_first_name_statistics,
            commands::db_get_surname_statistics,
            commands::db_create_player_indexes,
            commands::db_get_league_config,
            commands::db_save_league_config,
            commands::db_create_league_config_table,
            commands::db_create_league_metadata_table,
            commands::db_get_all_leagues,
            commands::db_get_active_league,
            commands::db_create_league,
            commands::db_set_active_league,
            commands::db_delete_league,
            // Backup Commands
            commands::db_create_backup,
            commands::db_list_backups,
            commands::db_restore_backup,
            commands::db_cleanup_old_backups,
            // Census Import Commands
            commands::db_import_all_census_files,
            commands::db_import_census_file,
            // Whites Directory Matching Commands
            commands::whites_get_unmatched_entries,
            commands::whites_get_player_candidates,
            commands::whites_search_player_candidates,
            commands::whites_generate_suggestions,
            commands::whites_accept_match,
            commands::whites_reject_match,
            commands::whites_get_stats,
            commands::get_database_matching_stats,
            commands::count_abraham_adams,
            commands::db_list_all_tables,
            commands::db_get_table_schema,
            commands::get_female_footballers_count,
            commands::delete_female_footballers,
            // Parish Management Commands
            commands::db_get_all_parishes,
            commands::db_assign_postcode_to_parish,
            commands::db_assign_parish_players_to_clubs,
            commands::db_check_club_assignments,
            commands::db_update_club_additional_postcode,
            commands::db_update_parish_additional_postcode,
            commands::check_temp_db_data,
            // Match Engine Commands
            commands::simulate_match_batch,
            commands::simulate_match_live,
            commands::simulate_sheffield_1867_match,
            commands::get_available_formations,
            commands::get_formation_details,
            commands::start_quick_match,
            commands::start_live_match,
            commands::get_match_replay_data,
            // Challenge Invitation System Commands
            commands::send_challenge_letter,
            commands::get_news_for_date,
            commands::get_all_news,
            commands::get_unread_news_count,
            commands::mark_news_as_read,
            commands::create_match_confirmed_news,
            commands::get_invitation_by_id,
            commands::check_pending_invitation_responses,
            commands::wipe_master_challenge_data,
            // Trial Session Commands
            commands::generate_trial_players,
            commands::schedule_trial_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
