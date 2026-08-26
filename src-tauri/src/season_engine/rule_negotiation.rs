/// Rule Negotiation System: Inter-code match rule agreements
///
/// Historical context:
/// - Different codes existed 1858-1877 (Sheffield, Nottingham, Cambridge, etc.)
/// - Before FA standardization, clubs negotiated which rules to use
/// - Common compromises: half under each code, or hybrid ruleset
/// - Higher prestige clubs often dictated terms

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleNegotiation {
    pub match_id: String,
    pub home_club_id: String,
    pub away_club_id: String,
    pub home_club_code: FootballCode,
    pub away_club_code: FootballCode,
    pub negotiated_ruleset: NegotiatedRuleset,
    pub negotiation_type: NegotiationType,
    pub narrative: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FootballCode {
    SheffieldRules { year: i64 },
    NottinghamRules { year: i64 },
    CambridgeRules { year: i64 },
    FA_Rules { year: i64 },
    RugbySchoolRules { year: i64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NegotiationType {
    HomeRulesOnly,
    AwayRulesOnly,
    HalfAndHalf,
    HybridCompromise,
    NeutralRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiatedRuleset {
    pub base_code: FootballCode,
    pub modifications: Vec<RuleModification>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleModification {
    pub rule_name: String,
    pub modification_type: ModificationType,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    Enable,
    Disable,
    Modify,
}

impl RuleNegotiation {
    /// Negotiate rules based on club prestige and codes
    pub async fn negotiate(
        pool: &SqlitePool,
        match_id: String,
        home_club_id: String,
        away_club_id: String,
    ) -> Result<Self, String> {
        // Get club information including preferred codes
        let home_code = Self::get_club_code(pool, &home_club_id).await?;
        let away_code = Self::get_club_code(pool, &away_club_id).await?;

        // If same code, no negotiation needed
        if Self::same_code(&home_code, &away_code) {
            return Ok(Self {
                match_id: match_id.clone(),
                home_club_id,
                away_club_id,
                home_club_code: home_code.clone(),
                away_club_code: away_code.clone(),
                negotiated_ruleset: NegotiatedRuleset {
                    base_code: home_code,
                    modifications: vec![],
                    description: "Standard rules".to_string(),
                },
                negotiation_type: NegotiationType::HomeRulesOnly,
                narrative: "Both clubs play under the same code. No negotiation required.".to_string(),
            });
        }

        // Get club prestige/reputation
        let home_prestige = Self::get_club_prestige(pool, &home_club_id).await?;
        let away_prestige = Self::get_club_prestige(pool, &away_club_id).await?;

        // Determine negotiation outcome
        let (negotiation_type, negotiated_ruleset) =
            Self::determine_negotiation_outcome(&home_code, &away_code, home_prestige, away_prestige);

        let narrative = Self::generate_narrative(
            &negotiation_type,
            &home_code,
            &away_code,
            &negotiated_ruleset
        );

        Ok(Self {
            match_id,
            home_club_id,
            away_club_id,
            home_club_code: home_code,
            away_club_code: away_code,
            negotiated_ruleset,
            negotiation_type,
            narrative,
        })
    }

    async fn get_club_code(pool: &SqlitePool, club_id: &str) -> Result<FootballCode, String> {
        let (code, year) = sqlx::query_as::<_, (String, i64)>(
            "SELECT COALESCE(preferred_code, 'sheffield_rules'), COALESCE(code_year, 1867) FROM sheffield_clubs WHERE id = ?"
        )
        .bind(club_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Self::parse_code(&code, year))
    }

    async fn get_club_prestige(pool: &SqlitePool, club_id: &str) -> Result<i32, String> {
        // Use reputation or calculate from historical performance
        sqlx::query_scalar::<_, i32>(
            "SELECT COALESCE(overall_rating, 50) FROM (
                SELECT AVG(overall_rating) as overall_rating
                FROM sheffield_footballers
                WHERE club_id = ?
                LIMIT 1
            )"
        )
        .bind(club_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    fn parse_code(code_str: &str, year: i64) -> FootballCode {
        match code_str {
            "sheffield_rules" => FootballCode::SheffieldRules { year },
            "nottingham_rules" => FootballCode::NottinghamRules { year },
            "cambridge_rules" => FootballCode::CambridgeRules { year },
            "fa_rules" => FootballCode::FA_Rules { year },
            "rugby_rules" => FootballCode::RugbySchoolRules { year },
            _ => FootballCode::SheffieldRules { year },
        }
    }

    fn same_code(code1: &FootballCode, code2: &FootballCode) -> bool {
        std::mem::discriminant(code1) == std::mem::discriminant(code2)
    }

    fn determine_negotiation_outcome(
        home_code: &FootballCode,
        away_code: &FootballCode,
        home_prestige: i32,
        away_prestige: i32,
    ) -> (NegotiationType, NegotiatedRuleset) {
        // Prestige difference determines negotiating power
        let prestige_diff = home_prestige - away_prestige;

        if prestige_diff > 15 {
            // Home team dictates
            (
                NegotiationType::HomeRulesOnly,
                NegotiatedRuleset {
                    base_code: home_code.clone(),
                    modifications: vec![],
                    description: format!("{} rules", Self::code_name(home_code)),
                }
            )
        } else if prestige_diff < -15 {
            // Away team dictates
            (
                NegotiationType::AwayRulesOnly,
                NegotiatedRuleset {
                    base_code: away_code.clone(),
                    modifications: vec![],
                    description: format!("{} rules", Self::code_name(away_code)),
                }
            )
        } else {
            // Compromise required
            let ruleset = Self::create_hybrid_ruleset(home_code, away_code);
            (NegotiationType::HybridCompromise, ruleset)
        }
    }

    fn create_hybrid_ruleset(
        home_code: &FootballCode,
        away_code: &FootballCode
    ) -> NegotiatedRuleset {
        match (home_code, away_code) {
            // Sheffield vs Nottingham
            (FootballCode::SheffieldRules { year }, FootballCode::NottinghamRules { .. }) |
            (FootballCode::NottinghamRules { .. }, FootballCode::SheffieldRules { year }) => {
                let modifications = vec![
                    RuleModification {
                        rule_name: "Rouge Scoring".to_string(),
                        modification_type: ModificationType::Disable,
                        description: "Rouge scoring disabled by mutual agreement".to_string(),
                    },
                    RuleModification {
                        rule_name: "Fair Catch".to_string(),
                        modification_type: ModificationType::Enable,
                        description: "Nottingham's fair catch provision allowed".to_string(),
                    },
                ];

                NegotiatedRuleset {
                    base_code: FootballCode::SheffieldRules { year: *year },
                    modifications,
                    description: "Sheffield Rules base, no rouges, fair catch permitted".to_string(),
                }
            }

            // Sheffield vs Cambridge
            (FootballCode::SheffieldRules { year }, FootballCode::CambridgeRules { .. }) |
            (FootballCode::CambridgeRules { .. }, FootballCode::SheffieldRules { year }) => {
                let modifications = vec![
                    RuleModification {
                        rule_name: "Corner Kicks".to_string(),
                        modification_type: ModificationType::Enable,
                        description: "Sheffield's corner-kick innovation adopted".to_string(),
                    },
                    RuleModification {
                        rule_name: "Throw-ins".to_string(),
                        modification_type: ModificationType::Modify,
                        description: "Cambridge one-handed throw method used".to_string(),
                    },
                ];

                NegotiatedRuleset {
                    base_code: FootballCode::SheffieldRules { year: *year },
                    modifications,
                    description: "Sheffield Rules with Cambridge throw-in method".to_string(),
                }
            }

            // Sheffield vs FA Rules
            (FootballCode::SheffieldRules { .. }, FootballCode::FA_Rules { year }) |
            (FootballCode::FA_Rules { year }, FootballCode::SheffieldRules { .. }) => {
                NegotiatedRuleset {
                    base_code: FootballCode::FA_Rules { year: *year },
                    modifications: vec![],
                    description: "FA Rules (neutral compromise)".to_string(),
                }
            }

            // Default: use home rules
            _ => NegotiatedRuleset {
                base_code: home_code.clone(),
                modifications: vec![],
                description: format!("{} rules by default", Self::code_name(home_code)),
            },
        }
    }

    fn generate_narrative(
        negotiation_type: &NegotiationType,
        home_code: &FootballCode,
        away_code: &FootballCode,
        ruleset: &NegotiatedRuleset,
    ) -> String {
        match negotiation_type {
            NegotiationType::HomeRulesOnly => {
                format!(
                    "The match will be played entirely under {}, as dictated by the home club.",
                    Self::code_name(home_code)
                )
            }
            NegotiationType::AwayRulesOnly => {
                format!(
                    "The visiting club insisted upon {} for this fixture.",
                    Self::code_name(away_code)
                )
            }
            NegotiationType::HalfAndHalf => {
                format!(
                    "A novel compromise: the first half under {}, the second under {}.",
                    Self::code_name(home_code),
                    Self::code_name(away_code)
                )
            }
            NegotiationType::HybridCompromise => {
                format!(
                    "After cordial negotiation, a hybrid ruleset was agreed: {}",
                    ruleset.description
                )
            }
            NegotiationType::NeutralRules => {
                format!(
                    "Both clubs consented to neutral {} for fair play.",
                    ruleset.description
                )
            }
        }
    }

    fn code_name(code: &FootballCode) -> String {
        match code {
            FootballCode::SheffieldRules { year } => format!("Sheffield Rules ({})", year),
            FootballCode::NottinghamRules { year } => format!("Nottingham Rules ({})", year),
            FootballCode::CambridgeRules { year } => format!("Cambridge Rules ({})", year),
            FootballCode::FA_Rules { year } => format!("FA Rules ({})", year),
            FootballCode::RugbySchoolRules { year } => format!("Rugby School Rules ({})", year),
        }
    }

    /// Get ruleset as JSON for storage
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.negotiated_ruleset).unwrap_or_default()
    }

    /// Check if rouge scoring is enabled in negotiated rules
    pub fn rouge_enabled(&self) -> bool {
        // Check if rouge was disabled
        for modification in &self.negotiated_ruleset.modifications {
            if modification.rule_name == "Rouge Scoring"
                && matches!(modification.modification_type, ModificationType::Disable) {
                return false;
            }
        }

        // Otherwise check base code
        matches!(self.negotiated_ruleset.base_code, FootballCode::SheffieldRules { year } if year >= 1862 && year <= 1868)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_code_detection() {
        let code1 = FootballCode::SheffieldRules { year: 1867 };
        let code2 = FootballCode::SheffieldRules { year: 1870 };
        let code3 = FootballCode::NottinghamRules { year: 1862 };

        assert!(RuleNegotiation::same_code(&code1, &code2));
        assert!(!RuleNegotiation::same_code(&code1, &code3));
    }

    #[test]
    fn test_hybrid_ruleset_creation() {
        let home_code = FootballCode::SheffieldRules { year: 1867 };
        let away_code = FootballCode::NottinghamRules { year: 1862 };

        let ruleset = RuleNegotiation::create_hybrid_ruleset(&home_code, &away_code);

        assert!(ruleset.modifications.len() >= 1);
        assert!(ruleset.modifications.iter().any(|m| m.rule_name == "Rouge Scoring"));
    }
}
