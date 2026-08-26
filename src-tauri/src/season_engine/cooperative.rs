/// Co-operative Movement System: Brightside and Carbrook Co-operative Society
///
/// Historical context:
/// - Founded 1868 by William Shaw and artisans in Carbrook
/// - Democratically-run by members
/// - Focus on fair trade, proper labour conditions, social advancement
/// - Rapid growth: 12,000 members by 1900, £300,000 annual sales
///
/// Gameplay integration:
/// - Kit purchase and donation
/// - Equipment purchase (balls, goalposts, etc.)
/// - Meeting hall rentals
/// - Cooperative dividend system
/// - Social events and education
/// - Member benefits for players/clubs

use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperativeSystem {
    pub is_founded: bool,
    pub founding_date: Option<NaiveDate>,
    pub membership_count: i32,
    pub annual_sales: i32,  // In pounds sterling
    pub dividend_rate: f32,  // Percentage returned to members
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperativeMembership {
    pub club_id: String,
    pub club_name: String,
    pub joined_date: NaiveDate,
    pub member_number: i32,
    pub share_capital: i32,  // In shillings
    pub total_purchases: i32,  // Lifetime purchases in shillings
    pub dividend_earned: i32,  // Total dividend in shillings
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperativeTransaction {
    pub transaction_id: String,
    pub club_id: String,
    pub transaction_date: NaiveDate,
    pub transaction_type: TransactionType,
    pub item_description: String,
    pub cost_shillings: i32,
    pub dividend_eligible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    KitPurchase,
    EquipmentPurchase,
    BallPurchase,
    GoalpostRepair,
    MeetingHallRental,
    RefreshmentsPurchase,
    EducationalLecture,
    SocialEvent,
    Donation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperativeEvent {
    pub event_type: CooperativeEventType,
    pub date: NaiveDate,
    pub description: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CooperativeEventType {
    Founding,
    NewShopOpening,
    MembershipMilestone,
    DividendDistribution,
    AnnualMeeting,
    LanternLecture,
    EducationalClass,
    SocialGathering,
}

impl Default for CooperativeSystem {
    fn default() -> Self {
        Self {
            is_founded: false,
            founding_date: None,
            membership_count: 0,
            annual_sales: 0,
            dividend_rate: 0.0,
        }
    }
}

impl CooperativeSystem {
    /// Check if cooperative should be founded (1868 or later)
    pub fn check_founding(&mut self, current_date: NaiveDate) -> Option<CooperativeEvent> {
        if self.is_founded {
            return None;
        }

        let year = current_date.year();

        // Historical founding: 1868
        if year >= 1868 {
            self.is_founded = true;
            self.founding_date = Some(current_date);
            self.membership_count = 50;  // Initial founding members
            self.dividend_rate = 2.5;  // Initial 2.5% dividend

            Some(CooperativeEvent {
                event_type: CooperativeEventType::Founding,
                date: current_date,
                description: "The Brightside and Carbrook Co-operative Society has been founded by Mr. William Shaw and a group of artisans in Carbrook. The Society opens its first shop, dedicated to fair trade and proper labour conditions.".to_string(),
                impact: "Football clubs may now purchase kits, equipment, and supplies from the Co-operative, earning dividend on purchases and supporting Trade Union labour conditions.".to_string(),
            })
        } else {
            None
        }
    }

    /// Advance cooperative one year (growth simulation)
    pub fn advance_year(&mut self, current_year: i64) -> Vec<CooperativeEvent> {
        if !self.is_founded {
            return vec![];
        }

        let mut events = vec![];
        let years_since_founding = current_year - 1868;

        if years_since_founding < 0 {
            return vec![];
        }

        // Historical growth pattern: exponential to 12,000 by 1900 (32 years)
        // Using growth formula: members = 50 * e^(0.17 * years)
        let growth_factor = 1.0 + (years_since_founding as f32 * 0.17);
        self.membership_count = (50.0 * growth_factor) as i32;
        self.membership_count = self.membership_count.min(12000);  // Cap at historical 12,000

        // Sales growth proportional to membership
        self.annual_sales = (self.membership_count as f32 * 25.0) as i32;  // ~£25 per member

        // Dividend rate improves with scale
        self.dividend_rate = 2.5 + (years_since_founding as f32 * 0.1).min(5.0);  // Max 7.5%

        // Milestone events
        if self.membership_count >= 1000 && self.membership_count < 1500 {
            events.push(CooperativeEvent {
                event_type: CooperativeEventType::MembershipMilestone,
                date: NaiveDate::from_ymd_opt(current_year as i32, 12, 31).unwrap(),
                description: format!(
                    "The Co-operative Society celebrates reaching {} members, a testament to the success of fair trade principles.",
                    self.membership_count
                ),
                impact: "Dividend rate increased to {:.1}%".to_string().replace("{:.1}", &format!("{:.1}", self.dividend_rate)),
            });
        }

        events
    }

    /// Calculate dividend for a club's purchases
    pub fn calculate_dividend(&self, purchase_amount_shillings: i32) -> i32 {
        if !self.is_founded {
            return 0;
        }

        let dividend_pounds = (purchase_amount_shillings as f32 / 20.0) * (self.dividend_rate / 100.0);
        (dividend_pounds * 20.0) as i32  // Convert back to shillings
    }

    /// Register a club as a Co-operative member
    pub async fn register_club_membership(
        &self,
        pool: &SqlitePool,
        club_id: &str,
        club_name: &str,
        current_date: NaiveDate,
    ) -> Result<CooperativeMembership, String> {
        if !self.is_founded {
            return Err("Co-operative not yet founded".to_string());
        }

        // Check if already member
        let existing = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM sheffield_cooperative_memberships WHERE club_id = ?"
        )
        .bind(club_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        if existing > 0 {
            return Err("Club already a member".to_string());
        }

        // Get next member number
        let member_number: i32 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(member_number), 0) + 1 FROM sheffield_cooperative_memberships"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Initial share capital: 1 pound (20 shillings)
        let share_capital = 20;

        sqlx::query(
            r#"
            INSERT INTO sheffield_cooperative_memberships
            (club_id, club_name, joined_date, member_number, share_capital, total_purchases, dividend_earned, is_active)
            VALUES (?, ?, ?, ?, ?, 0, 0, 1)
            "#
        )
        .bind(club_id)
        .bind(club_name)
        .bind(current_date.to_string())
        .bind(member_number)
        .bind(share_capital)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(CooperativeMembership {
            club_id: club_id.to_string(),
            club_name: club_name.to_string(),
            joined_date: current_date,
            member_number,
            share_capital,
            total_purchases: 0,
            dividend_earned: 0,
            is_active: true,
        })
    }

    /// Process a purchase transaction
    pub async fn process_purchase(
        &self,
        pool: &SqlitePool,
        club_id: &str,
        transaction_type: TransactionType,
        item_description: String,
        cost_shillings: i32,
        current_date: NaiveDate,
    ) -> Result<CooperativeTransaction, String> {
        if !self.is_founded {
            return Err("Co-operative not yet founded".to_string());
        }

        // Generate transaction ID
        let transaction_id = format!("coop-txn-{}-{}", current_date.format("%Y%m%d"), uuid::Uuid::new_v4());

        // Check if dividend eligible (purchases, not donations/rentals)
        let dividend_eligible = matches!(
            transaction_type,
            TransactionType::KitPurchase
                | TransactionType::EquipmentPurchase
                | TransactionType::BallPurchase
                | TransactionType::RefreshmentsPurchase
        );

        // Record transaction
        sqlx::query(
            r#"
            INSERT INTO sheffield_cooperative_transactions
            (transaction_id, club_id, transaction_date, transaction_type, item_description, cost_shillings, dividend_eligible)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&transaction_id)
        .bind(club_id)
        .bind(current_date.to_string())
        .bind(format!("{:?}", transaction_type))
        .bind(&item_description)
        .bind(cost_shillings)
        .bind(dividend_eligible)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Update club's total purchases
        sqlx::query(
            "UPDATE sheffield_cooperative_memberships SET total_purchases = total_purchases + ? WHERE club_id = ?"
        )
        .bind(cost_shillings)
        .bind(club_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Calculate and add dividend if eligible
        if dividend_eligible {
            let dividend = self.calculate_dividend(cost_shillings);
            sqlx::query(
                "UPDATE sheffield_cooperative_memberships SET dividend_earned = dividend_earned + ? WHERE club_id = ?"
            )
            .bind(dividend)
            .bind(club_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(CooperativeTransaction {
            transaction_id,
            club_id: club_id.to_string(),
            transaction_date: current_date,
            transaction_type,
            item_description,
            cost_shillings,
            dividend_eligible,
        })
    }

    /// Get available items for purchase
    pub fn get_shop_catalog(&self, current_year: i64) -> Vec<ShopItem> {
        if !self.is_founded {
            return vec![];
        }

        vec![
            // Kits and clothing
            ShopItem {
                category: "Kits".to_string(),
                name: "Full Football Kit (Jersey, Knickerbockers, Stockings)".to_string(),
                description: "Complete playing outfit in club colours. Made under proper Trade Union conditions.".to_string(),
                cost_shillings: 25,  // ~£1 5s
                available_from_year: 1868,
            },
            ShopItem {
                category: "Kits".to_string(),
                name: "Jersey Only".to_string(),
                description: "Woollen jersey in club colours.".to_string(),
                cost_shillings: 10,
                available_from_year: 1868,
            },
            ShopItem {
                category: "Kits".to_string(),
                name: "Knickerbockers".to_string(),
                description: "Cotton knickerbockers suitable for football.".to_string(),
                cost_shillings: 8,
                available_from_year: 1868,
            },
            ShopItem {
                category: "Kits".to_string(),
                name: "Woollen Stockings (pair)".to_string(),
                description: "Thick woollen stockings.".to_string(),
                cost_shillings: 3,
                available_from_year: 1868,
            },

            // Equipment
            ShopItem {
                category: "Equipment".to_string(),
                name: "Leather Football".to_string(),
                description: "Hand-stitched leather football, properly inflated.".to_string(),
                cost_shillings: 12,
                available_from_year: 1868,
            },
            ShopItem {
                category: "Equipment".to_string(),
                name: "Goal Posts (pair with crossbar)".to_string(),
                description: "Wooden goal posts with crossbar, regulation size.".to_string(),
                cost_shillings: 60,  // £3
                available_from_year: 1868,
            },
            ShopItem {
                category: "Equipment".to_string(),
                name: "Corner Flags (set of 4)".to_string(),
                description: "Corner marking flags on poles.".to_string(),
                cost_shillings: 8,
                available_from_year: 1868,
            },
            ShopItem {
                category: "Equipment".to_string(),
                name: "Shin Guards (pair)".to_string(),
                description: "Protective shin guards. A modern innovation!".to_string(),
                cost_shillings: 6,
                available_from_year: 1874,  // Historical: shin guards introduced 1874
            },

            // Facilities
            ShopItem {
                category: "Facilities".to_string(),
                name: "Meeting Hall Rental (per month)".to_string(),
                description: "Use of Co-operative hall for club meetings and social events.".to_string(),
                cost_shillings: 10,
                available_from_year: 1868,
            },

            // Refreshments
            ShopItem {
                category: "Refreshments".to_string(),
                name: "Tea and Refreshments (per match)".to_string(),
                description: "Tea, bread, and refreshments for players after matches.".to_string(),
                cost_shillings: 5,
                available_from_year: 1868,
            },

            // Education and social
            ShopItem {
                category: "Education".to_string(),
                name: "Lantern Lecture Ticket".to_string(),
                description: "Admission to educational lantern lecture series.".to_string(),
                cost_shillings: 1,
                available_from_year: 1868,
            },
        ]
    }

    /// Generate random cooperative event (for newspaper)
    pub fn generate_event(&self, current_date: NaiveDate) -> Option<CooperativeEvent> {
        if !self.is_founded {
            return None;
        }

        let mut rng = rand::thread_rng();

        // 20% chance of event on any given day
        if rng.gen::<f32>() > 0.20 {
            return None;
        }

        let events = vec![
            (
                CooperativeEventType::LanternLecture,
                "Lantern Lecture at Co-operative Hall",
                "A lantern lecture on 'The Principles of Fair Trade' will be delivered at the Co-operative Hall on Saturday evening. Members and friends are cordially invited.",
            ),
            (
                CooperativeEventType::EducationalClass,
                "Educational Classes Commence",
                "The Co-operative Society announces the commencement of educational classes for working men, covering arithmetic, reading, and the principles of mutual aid.",
            ),
            (
                CooperativeEventType::SocialGathering,
                "Co-operative Social Evening",
                "A social evening for members of the Co-operative will be held at the Brightside Hall, with refreshments and entertainment provided.",
            ),
            (
                CooperativeEventType::NewShopOpening,
                "New Co-operative Branch Opens",
                "The Co-operative Society has opened a new branch shop, extending its service to a wider district and offering members greater convenience.",
            ),
            (
                CooperativeEventType::AnnualMeeting,
                "Co-operative Annual General Meeting",
                "The Annual General Meeting of the Brightside and Carbrook Co-operative Society will take place on Monday evening. All members are urged to attend to hear reports of the Society's continued prosperity.",
            ),
        ];

        let (event_type, headline, description) = &events[rng.gen_range(0..events.len())];

        Some(CooperativeEvent {
            event_type: event_type.clone(),
            date: current_date,
            description: description.to_string(),
            impact: String::new(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItem {
    pub category: String,
    pub name: String,
    pub description: String,
    pub cost_shillings: i32,
    pub available_from_year: i64,
}

impl ShopItem {
    pub fn format_price(&self) -> String {
        let pounds = self.cost_shillings / 20;
        let shillings = self.cost_shillings % 20;

        if pounds > 0 && shillings > 0 {
            format!("£{} {}s", pounds, shillings)
        } else if pounds > 0 {
            format!("£{}", pounds)
        } else {
            format!("{}s", shillings)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooperative_founding() {
        let mut coop = CooperativeSystem::default();
        let date_1867 = NaiveDate::from_ymd_opt(1867, 10, 1).unwrap();
        let date_1868 = NaiveDate::from_ymd_opt(1868, 3, 15).unwrap();

        // Should not found before 1868
        assert!(coop.check_founding(date_1867).is_none());
        assert!(!coop.is_founded);

        // Should found in 1868
        let event = coop.check_founding(date_1868);
        assert!(event.is_some());
        assert!(coop.is_founded);
        assert_eq!(coop.membership_count, 50);
    }

    #[test]
    fn test_dividend_calculation() {
        let mut coop = CooperativeSystem::default();
        coop.is_founded = true;
        coop.dividend_rate = 5.0;  // 5%

        // Purchase of 20 shillings (£1)
        let dividend = coop.calculate_dividend(20);
        assert_eq!(dividend, 1);  // 5% of £1 = 1 shilling

        // Purchase of 100 shillings (£5)
        let dividend = coop.calculate_dividend(100);
        assert_eq!(dividend, 5);  // 5% of £5 = 5 shillings
    }

    #[test]
    fn test_growth_simulation() {
        let mut coop = CooperativeSystem::default();
        coop.is_founded = true;

        // Advance 10 years
        coop.advance_year(1878);
        assert!(coop.membership_count > 50);
        assert!(coop.annual_sales > 0);
        assert!(coop.dividend_rate > 2.5);
    }

    #[test]
    fn test_shop_catalog() {
        let mut coop = CooperativeSystem::default();
        coop.is_founded = true;

        let catalog_1868 = coop.get_shop_catalog(1868);
        let catalog_1874 = coop.get_shop_catalog(1874);

        // Shin guards not available until 1874
        assert!(!catalog_1868.iter().any(|item| item.name.contains("Shin Guards")));
        assert!(catalog_1874.iter().any(|item| item.name.contains("Shin Guards")));
    }
}
