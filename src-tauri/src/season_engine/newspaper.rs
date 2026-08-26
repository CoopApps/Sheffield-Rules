/// Victorian Newspaper System: Daily newspaper generation with match reports and local news
///
/// Features:
/// - Authentic Victorian vocabulary and phrasing
/// - Match reports with narrative flavor
/// - Local news stories (Sheffield happenings)
/// - Multiple article types (reports, opinion, fixtures, standings)
/// - Template-based generation with variety

use serde::{Deserialize, Serialize};
use rand::Rng;
use chrono::{NaiveDate, Datelike, Weekday};
use super::local_events::{LocalEvent, LocalEventType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewspaperSystem {
    pub publication_name: String,
    pub city: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewspaperEdition {
    pub publication_name: String,
    pub edition_date: String,
    pub headline: String,
    pub articles: Vec<Article>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub article_type: ArticleType,
    pub headline: String,
    pub body: String,
    pub byline: Option<String>,
    pub clubs_mentioned: Vec<String>,
    pub players_mentioned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArticleType {
    MatchReport,
    Preview,
    Opinion,
    LocalNews,
    Standings,
    Fixtures,
    CupDraw,
    RuleDiscussion,
}

impl Default for NewspaperSystem {
    fn default() -> Self {
        Self {
            publication_name: "The Sheffield Independent".to_string(),
            city: "Sheffield".to_string(),
        }
    }
}

impl NewspaperSystem {
    pub fn new(publication_name: String, city: String) -> Self {
        Self {
            publication_name,
            city,
        }
    }

    /// Generate a complete daily edition
    pub fn generate_daily_edition(
        &self,
        date: NaiveDate,
        match_results: Vec<MatchResult>,
        upcoming_fixtures: Vec<Fixture>,
        local_events: Vec<LocalEvent>,
        standings: Option<Vec<Standing>>,
    ) -> NewspaperEdition {
        let mut articles = vec![];

        // Lead story: Most significant match or local event
        if let Some(top_match) = match_results.first() {
            let match_report = self.generate_match_report(top_match, true);
            articles.push(match_report);
        } else if let Some(top_event) = local_events.first() {
            let local_article = self.generate_local_news_article(top_event);
            articles.push(local_article);
        }

        // Additional match reports (condensed)
        for result in match_results.iter().skip(1).take(3) {
            articles.push(self.generate_match_report(result, false));
        }

        // Local news stories (2-3 stories)
        for event in local_events.iter().take(3) {
            articles.push(self.generate_local_news_article(event));
        }

        // League table (Mondays)
        if date.weekday() == chrono::Weekday::Mon {
            if let Some(table) = standings {
                articles.push(self.generate_standings_article(&table));
            }
        }

        // Fixtures (Saturdays)
        if date.weekday() == chrono::Weekday::Sat && !upcoming_fixtures.is_empty() {
            articles.push(self.generate_fixtures_article(&upcoming_fixtures));
        }

        let headline = articles.first()
            .map(|a| a.headline.clone())
            .unwrap_or_else(|| "FOOTBALL INTELLIGENCE".to_string());

        NewspaperEdition {
            publication_name: self.publication_name.clone(),
            edition_date: date.format("%A, %d %B %Y").to_string(),
            headline,
            articles,
        }
    }

    /// Generate a full match report with Victorian vocabulary
    fn generate_match_report(&self, result: &MatchResult, is_lead: bool) -> Article {
        let mut rng = rand::thread_rng();

        // Victorian opening phrases
        let openings = [
            "A capital exhibition of football was witnessed",
            "A most spirited contest took place",
            "The assembled spectators were treated to a fine display",
            "An excellent match was played",
            "A thrilling encounter unfolded",
        ];

        let opening = openings[rng.gen_range(0..openings.len())];

        // Weather integration
        let weather_desc = match result.weather.as_deref() {
            Some("clear") => "under clear skies",
            Some("overcast") => "beneath leaden skies",
            Some("light_rain") => "despite occasional showers",
            Some("heavy_rain") => "in most disagreeable conditions",
            Some("snow") => "amidst falling snow",
            Some("fog") => "through thick mist",
            _ => "in fair conditions",
        };

        // Determine narrative tone based on result
        let home_dominant = result.home_score > result.away_score;
        let score_diff = (result.home_score - result.away_score).abs();
        let close_match = score_diff <= 1;

        let body = if is_lead {
            self.generate_detailed_match_narrative(result, opening, weather_desc, home_dominant, close_match)
        } else {
            self.generate_condensed_match_narrative(result, opening, weather_desc)
        };

        let headline = if close_match {
            format!("{} EDGE THRILLING ENCOUNTER", result.home_club_name.to_uppercase())
        } else if home_dominant {
            format!("{} TRIUMPH IN CAPITAL FASHION", result.home_club_name.to_uppercase())
        } else {
            format!("{} SECURE FINE VICTORY", result.away_club_name.to_uppercase())
        };

        Article {
            article_type: ArticleType::MatchReport,
            headline,
            body,
            byline: Some("Our Football Correspondent".to_string()),
            clubs_mentioned: vec![result.home_club_id.clone(), result.away_club_id.clone()],
            players_mentioned: result.goal_scorers.clone(),
        }
    }

    fn generate_detailed_match_narrative(
        &self,
        result: &MatchResult,
        opening: &str,
        weather: &str,
        home_dominant: bool,
        close: bool,
    ) -> String {
        let mut narrative = String::new();

        // Opening paragraph
        narrative.push_str(&format!(
            "\t{} yesterday at {}, {}, when {} met {} before a crowd \
             numbering upwards of {} souls. The contest concluded with a score of {} to {} in favour of {}.\n\n",
            opening,
            result.venue,
            weather,
            result.home_club_name,
            result.away_club_name,
            result.attendance,
            if home_dominant { result.home_score } else { result.away_score },
            if home_dominant { result.away_score } else { result.home_score },
            if home_dominant { &result.home_club_name } else { &result.away_club_name }
        ));

        // Match description paragraph
        if close {
            narrative.push_str(&format!(
                "\tThe game was contested with great spirit throughout, both elevens displaying \
                 admirable skill and determination. The fortunes of the match swayed to and fro, \
                 with first one side and then the other appearing likely to secure victory. \
                 The {} men showed excellent combination play, whilst {} defended most resolutely.\n\n",
                if home_dominant { &result.home_club_name } else { &result.away_club_name },
                if home_dominant { &result.away_club_name } else { &result.home_club_name }
            ));
        } else {
            narrative.push_str(&format!(
                "\tThe victors dominated proceedings from the outset, their superior play being \
                 manifest throughout. {} displayed capital form, their forwards combining \
                 splendidly and their backs proving well-nigh impenetrable. The {} eleven, \
                 whilst showing pluck, were quite outmatched on the day.\n\n",
                if home_dominant { &result.home_club_name } else { &result.away_club_name },
                if home_dominant { &result.away_club_name } else { &result.home_club_name }
            ));
        }

        // Goals paragraph
        if !result.goal_scorers.is_empty() {
            narrative.push_str("\t");
            for (i, scorer) in result.goal_scorers.iter().enumerate() {
                if i > 0 {
                    narrative.push_str(", whilst ");
                }
                narrative.push_str(&self.generate_goal_description(scorer));
            }
            narrative.push_str(".\n\n");
        }

        // Rouge mention (if applicable)
        if result.home_rouges > 0 || result.away_rouges > 0 {
            narrative.push_str(&format!(
                "\tIn accordance with the Sheffield Rules, rouges were scored to the number of {} \
                 by {} and {} by {}, though these proved of no consequence given the goals obtained.\n\n",
                result.home_rouges,
                result.home_club_name,
                result.away_rouges,
                result.away_club_name
            ));
        }

        // Closing remarks
        narrative.push_str(&format!(
            "\tThe match was conducted in a most sporting manner, and reflected great credit \
             upon both clubs. We understand a return fixture is anticipated with considerable interest."
        ));

        narrative
    }

    fn generate_condensed_match_narrative(
        &self,
        result: &MatchResult,
        opening: &str,
        weather: &str,
    ) -> String {
        format!(
            "\t{} at {}, {}, between {} and {}. The home side prevailed by {} goals to {}, \
             {} securing the points before {} spectators. The play was of good quality throughout.",
            opening,
            result.venue,
            weather,
            result.home_club_name,
            result.away_club_name,
            result.home_score,
            result.away_score,
            if result.home_score > result.away_score { &result.home_club_name } else { &result.away_club_name },
            result.attendance
        )
    }

    fn generate_goal_description(&self, scorer_name: &str) -> String {
        let mut rng = rand::thread_rng();
        let descriptions = [
            format!("{} notched a capital goal", scorer_name),
            format!("{} found the net with a well-directed effort", scorer_name),
            format!("{} scored in fine fashion", scorer_name),
            format!("{} secured a goal after splendid play", scorer_name),
            format!("{} registered a fine goal", scorer_name),
        ];
        descriptions[rng.gen_range(0..descriptions.len())].clone()
    }

    /// Generate local news article with Victorian flavor
    fn generate_local_news_article(&self, event: &LocalEvent) -> Article {
        let body = match &event.event_type {
            LocalEventType::IndustrialNews => self.generate_industrial_news(&event.details),
            LocalEventType::CivicAnnouncement => self.generate_civic_news(&event.details),
            LocalEventType::Transport => self.generate_transport_news(&event.details),
            LocalEventType::Weather => self.generate_weather_news(&event.details),
            LocalEventType::Entertainment => self.generate_entertainment_news(&event.details),
            LocalEventType::Crime => self.generate_crime_news(&event.details),
            LocalEventType::Society => self.generate_society_news(&event.details),
            LocalEventType::Cooperative => self.generate_cooperative_news(&event.details),
        };

        Article {
            article_type: ArticleType::LocalNews,
            headline: event.headline.clone(),
            body,
            byline: Some("Our Local Correspondent".to_string()),
            clubs_mentioned: vec![],
            players_mentioned: vec![],
        }
    }

    fn generate_industrial_news(&self, details: &str) -> String {
        format!(
            "\tWe are informed that {}. The development is considered most promising \
             for the continued prosperity of our town. The proprietors express confidence \
             that this endeavour shall provide gainful employment for numerous hands.",
            details
        )
    }

    fn generate_civic_news(&self, details: &str) -> String {
        format!(
            "\tThe Town Council has announced that {}. The measure has been received \
             with approbation by the citizens, who anticipate considerable benefit therefrom. \
             The Aldermen deserve commendation for their progressive spirit.",
            details
        )
    }

    fn generate_transport_news(&self, details: &str) -> String {
        format!(
            "\tPassengers are advised that the Railway Company reports {}. \
             Travellers are requested to make allowance for these circumstances when \
             planning their journeys. The company expresses regret for any inconvenience occasioned.",
            details
        )
    }

    fn generate_weather_news(&self, details: &str) -> String {
        format!(
            "\tThe recent atmospheric conditions have been most remarkable. {}. \
             Our meteorological correspondent advises that such phenomena, whilst unusual, \
             are not unprecedented at this season. Citizens are counselled to take appropriate precautions.",
            details
        )
    }

    fn generate_entertainment_news(&self, details: &str) -> String {
        format!(
            "\tThe Theatre Royal announces that {}. Patrons are assured of a most \
             agreeable evening's entertainment. Tickets may be secured at the usual agencies.",
            details
        )
    }

    fn generate_crime_news(&self, details: &str) -> String {
        format!(
            "\tThe magistrates have adjudicated upon a case wherein {}. \
             The accused was found guilty and sentenced accordingly. The bench expressed \
             their displeasure at such conduct and hope the sentence serves as a deterrent.",
            details
        )
    }

    fn generate_society_news(&self, details: &str) -> String {
        format!(
            "\tWe have the pleasure to announce that {}. The event was attended \
             by persons of considerable distinction, and was conducted with great propriety. \
             All present pronounced it a most satisfactory occasion.",
            details
        )
    }

    fn generate_cooperative_news(&self, details: &str) -> String {
        format!(
            "\t{}. This development is viewed with great satisfaction by all who recognise \
             the value of co-operative principles in promoting the welfare of working men and \
             ensuring fair conditions of trade. The Society continues to demonstrate that \
             commercial enterprise may be conducted with justice and mutual benefit, in \
             opposition to the evils of unscrupulous competition.",
            details
        )
    }

    fn generate_standings_article(&self, standings: &[Standing]) -> Article {
        let mut body = String::from("\tThe current standing of the clubs is as follows:\n\n");

        body.push_str("\tClub                           P    W   D   L   GF  GA  Pts\n");
        body.push_str("\t─────────────────────────────────────────────────────────\n");

        for standing in standings {
            body.push_str(&format!(
                "\t{:<28} {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>3}\n",
                standing.club_name,
                standing.played,
                standing.won,
                standing.drawn,
                standing.lost,
                standing.goals_for,
                standing.goals_against,
                standing.points
            ));
        }

        Article {
            article_type: ArticleType::Standings,
            headline: "FOOTBALL LEAGUE TABLE".to_string(),
            body,
            byline: None,
            clubs_mentioned: standings.iter().map(|s| s.club_id.clone()).collect(),
            players_mentioned: vec![],
        }
    }

    fn generate_fixtures_article(&self, fixtures: &[Fixture]) -> Article {
        let mut body = String::from("\tThe principal football engagements for the forthcoming week are:\n\n");

        for fixture in fixtures {
            body.push_str(&format!(
                "\t{} - {} v {} at {}\n",
                fixture.date,
                fixture.home_club_name,
                fixture.away_club_name,
                fixture.venue
            ));
        }

        body.push_str("\n\tFollowers of the pastime are assured of capital sport.");

        Article {
            article_type: ArticleType::Fixtures,
            headline: "FOOTBALL FIXTURES".to_string(),
            body,
            byline: None,
            clubs_mentioned: fixtures.iter()
                .flat_map(|f| vec![f.home_club_id.clone(), f.away_club_id.clone()])
                .collect(),
            players_mentioned: vec![],
        }
    }
}

// Supporting structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub home_club_id: String,
    pub away_club_id: String,
    pub home_club_name: String,
    pub away_club_name: String,
    pub home_score: i32,
    pub away_score: i32,
    pub home_rouges: i32,
    pub away_rouges: i32,
    pub venue: String,
    pub attendance: i32,
    pub weather: Option<String>,
    pub goal_scorers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub date: String,
    pub home_club_id: String,
    pub away_club_id: String,
    pub home_club_name: String,
    pub away_club_name: String,
    pub venue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standing {
    pub club_id: String,
    pub club_name: String,
    pub played: i32,
    pub won: i32,
    pub drawn: i32,
    pub lost: i32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub points: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_report_generation() {
        let newspaper = NewspaperSystem::default();
        let result = MatchResult {
            home_club_id: "sheffield-fc".to_string(),
            away_club_id: "hallam-fc".to_string(),
            home_club_name: "Sheffield FC".to_string(),
            away_club_name: "Hallam FC".to_string(),
            home_score: 3,
            away_score: 2,
            home_rouges: 1,
            away_rouges: 0,
            venue: "Bramall Lane".to_string(),
            attendance: 1200,
            weather: Some("clear".to_string()),
            goal_scorers: vec!["John Smith".to_string(), "William Brown".to_string()],
        };

        let article = newspaper.generate_match_report(&result, true);

        assert!(article.body.contains("Sheffield FC"));
        assert!(article.body.contains("Hallam FC"));
        assert!(article.body.len() > 200); // Detailed report
    }
}
