/// Sheffield & Hallamshire League (1867 Fantasy Mode)
/// Defines the complete league structure with all 346 clubs organized into 11 divisions
/// Each main club has a linked reserve team in a mirrored division structure

use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct LeagueDivision {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub region: Option<String>, // Geographic region for lower divisions
}

#[derive(Debug, Clone)]
pub struct LeagueClubAssignment {
    pub club_id: String,
    pub division_id: String,
    pub position_in_division: i32,
    pub is_reserve_team: bool,
    pub reserve_of_club_id: Option<String>,
}

pub struct PromotionRule {
    pub from_division_id: String,
    pub to_division_id: String,
    pub slots: i32,
}

/// Get all divisions for the Sheffield & Hallamshire League
pub fn get_all_divisions() -> Vec<LeagueDivision> {
    vec![
        // Main pyramid - Divisions 1-7D
        LeagueDivision {
            id: "div-1".to_string(),
            name: "First Division".to_string(),
            level: 1,
            region: None,
        },
        LeagueDivision {
            id: "div-2".to_string(),
            name: "Second Division".to_string(),
            level: 2,
            region: None,
        },
        LeagueDivision {
            id: "div-3".to_string(),
            name: "Third Division".to_string(),
            level: 3,
            region: None,
        },
        LeagueDivision {
            id: "div-4".to_string(),
            name: "Fourth Division".to_string(),
            level: 4,
            region: None,
        },
        LeagueDivision {
            id: "div-5a".to_string(),
            name: "Fifth Division A".to_string(),
            level: 5,
            region: None,
        },
        LeagueDivision {
            id: "div-5b".to_string(),
            name: "Fifth Division B".to_string(),
            level: 5,
            region: None,
        },
        LeagueDivision {
            id: "div-6a".to_string(),
            name: "Sixth Division West".to_string(),
            level: 6,
            region: Some("West".to_string()),
        },
        LeagueDivision {
            id: "div-6b".to_string(),
            name: "Sixth Division East".to_string(),
            level: 6,
            region: Some("East".to_string()),
        },
        LeagueDivision {
            id: "div-6c".to_string(),
            name: "Sixth Division North".to_string(),
            level: 6,
            region: Some("North".to_string()),
        },
        LeagueDivision {
            id: "div-6d".to_string(),
            name: "Sixth Division South".to_string(),
            level: 6,
            region: Some("South".to_string()),
        },
        LeagueDivision {
            id: "div-7a".to_string(),
            name: "Seventh Division West".to_string(),
            level: 7,
            region: Some("West".to_string()),
        },
        LeagueDivision {
            id: "div-7b".to_string(),
            name: "Seventh Division East".to_string(),
            level: 7,
            region: Some("East".to_string()),
        },
        LeagueDivision {
            id: "div-7c".to_string(),
            name: "Seventh Division North".to_string(),
            level: 7,
            region: Some("North".to_string()),
        },
        LeagueDivision {
            id: "div-7d".to_string(),
            name: "Seventh Division South".to_string(),
            level: 7,
            region: Some("South".to_string()),
        },
        // Reserve pyramid - Divisions 1-7D mirrored
        LeagueDivision {
            id: "res-div-1".to_string(),
            name: "Reserve Division 1".to_string(),
            level: 1,
            region: None,
        },
        LeagueDivision {
            id: "res-div-2".to_string(),
            name: "Reserve Division 2".to_string(),
            level: 2,
            region: None,
        },
        LeagueDivision {
            id: "res-div-3".to_string(),
            name: "Reserve Division 3".to_string(),
            level: 3,
            region: None,
        },
        LeagueDivision {
            id: "res-div-4".to_string(),
            name: "Reserve Division 4".to_string(),
            level: 4,
            region: None,
        },
        LeagueDivision {
            id: "res-div-5a".to_string(),
            name: "Reserve Division 5A".to_string(),
            level: 5,
            region: None,
        },
        LeagueDivision {
            id: "res-div-5b".to_string(),
            name: "Reserve Division 5B".to_string(),
            level: 5,
            region: None,
        },
        LeagueDivision {
            id: "res-div-6a".to_string(),
            name: "Reserve Division 6A".to_string(),
            level: 6,
            region: Some("West".to_string()),
        },
        LeagueDivision {
            id: "res-div-6b".to_string(),
            name: "Reserve Division 6B".to_string(),
            level: 6,
            region: Some("East".to_string()),
        },
        LeagueDivision {
            id: "res-div-6c".to_string(),
            name: "Reserve Division 6C".to_string(),
            level: 6,
            region: Some("North".to_string()),
        },
        LeagueDivision {
            id: "res-div-6d".to_string(),
            name: "Reserve Division 6D".to_string(),
            level: 6,
            region: Some("South".to_string()),
        },
        LeagueDivision {
            id: "res-div-7a".to_string(),
            name: "Reserve Division 7A".to_string(),
            level: 7,
            region: Some("West".to_string()),
        },
        LeagueDivision {
            id: "res-div-7b".to_string(),
            name: "Reserve Division 7B".to_string(),
            level: 7,
            region: Some("East".to_string()),
        },
        LeagueDivision {
            id: "res-div-7c".to_string(),
            name: "Reserve Division 7C".to_string(),
            level: 7,
            region: Some("North".to_string()),
        },
        LeagueDivision {
            id: "res-div-7d".to_string(),
            name: "Reserve Division 7D".to_string(),
            level: 7,
            region: Some("South".to_string()),
        },
    ]
}

/// Get the club-to-division assignments for the main league (186 clubs)
/// Returns tuples of: (club_id, division_id, position_in_division, reserve_team_club_id)
pub fn get_main_league_assignments() -> Vec<(String, String, i32, String)> {
    vec![
        // DIVISION 1 (16 clubs) - Earliest established clubs
        ("sheffield-fc".to_string(), "div-1".to_string(), 1, "sheffield-fc-reserves".to_string()),
        ("hallam-fc".to_string(), "div-1".to_string(), 2, "hallam-fc-reserves".to_string()),
        ("norfolk-fc".to_string(), "div-1".to_string(), 3, "norfolk-fc-juniors".to_string()),
        ("cemetery-road-church-fc".to_string(), "div-1".to_string(), 4, "cemetery-road-church-fc-reserves".to_string()),
        ("york-fc".to_string(), "div-1".to_string(), 5, "york-fc-juniors".to_string()),
        ("norton-fc".to_string(), "div-1".to_string(), 6, "norton-fc-b".to_string()),
        ("pitsmoor-fc".to_string(), "div-1".to_string(), 7, "pitsmoor-fc-b".to_string()),
        ("fir-vale-fc".to_string(), "div-1".to_string(), 8, "fir-vale-fc-b".to_string()),
        ("heeley-christ-church-fc".to_string(), "div-1".to_string(), 9, "heeley-christ-church-fc-reserves".to_string()),
        ("mackenzie-fc".to_string(), "div-1".to_string(), 10, "mackenzie-fc-second-xi".to_string()),
        ("milton-fc".to_string(), "div-1".to_string(), 11, "milton-fc-juniors".to_string()),
        ("howard-hill-steel-bank-fc".to_string(), "div-1".to_string(), 12, "howard-hill-steel-bank-fc-second-xi".to_string()),
        ("ranmoor-fc".to_string(), "div-1".to_string(), 13, "ranmoor-fc-reserves".to_string()),
        ("st-george-fc".to_string(), "div-1".to_string(), 14, "st-george-fc-reserves".to_string()),
        ("st-stephen-fc".to_string(), "div-1".to_string(), 15, "st-stephen-fc-reserves".to_string()),
        ("united-norfolk-fc".to_string(), "div-1".to_string(), 16, "united-norfolk-fc-reserves".to_string()),

        // DIVISION 2 (16 clubs) - 1863 onwards
        ("crabtree-fc".to_string(), "div-2".to_string(), 1, "crabtree-fc-b".to_string()),
        ("broomhall-fc".to_string(), "div-2".to_string(), 2, "broomhall-fc-b".to_string()),
        ("tudor-fc".to_string(), "div-2".to_string(), 3, "tudor-fc-b".to_string()),
        ("w-h-hutchinson-fc".to_string(), "div-2".to_string(), 4, "w-h-hutchinson-fc-second-xi".to_string()),
        ("hemsworth-fc".to_string(), "div-2".to_string(), 5, "hemsworth-fc-b".to_string()),
        ("united-mechanics".to_string(), "div-2".to_string(), 6, "united-mechanics-second-xi".to_string()),
        ("garrick-fc".to_string(), "div-2".to_string(), 7, "garrick-fc-juniors".to_string()),
        ("wellington-fc".to_string(), "div-2".to_string(), 8, "wellington-fc-juniors".to_string()),
        ("loxley-fc".to_string(), "div-2".to_string(), 9, "loxley-fc-juniors".to_string()),
        ("wednesday-fc".to_string(), "div-2".to_string(), 10, "wednesday-fc-b".to_string()),
        ("exchange-fc".to_string(), "div-2".to_string(), 11, "exchange-fc-b".to_string()),
        ("dore-fc".to_string(), "div-2".to_string(), 12, "dore-fc-juniors".to_string()),
        ("tapton-fc".to_string(), "div-2".to_string(), 13, "tapton-fc-b".to_string()),
        ("dronfield-fc".to_string(), "div-2".to_string(), 14, "dronfield-fc-b".to_string()),
        ("brincliffe-fc".to_string(), "div-2".to_string(), 15, "brincliffe-fc-b".to_string()),
        ("hanover-united-fc".to_string(), "div-2".to_string(), 16, "hanover-united-fc-b".to_string()),

        // DIVISION 3 (16 clubs) - 1868 onwards
        ("stannington-fc".to_string(), "div-3".to_string(), 1, "stannington-fc-juniors".to_string()),
        ("redhill-fc".to_string(), "div-3".to_string(), 2, "redhill-fc-b".to_string()),
        ("parkwood-springs-fc".to_string(), "div-3".to_string(), 3, "parkwood-springs-fc-b".to_string()),
        ("oxford-fc".to_string(), "div-3".to_string(), 4, "oxford-fc-b".to_string()),
        ("totley-fc".to_string(), "div-3".to_string(), 5, "totley-fc-juniors".to_string()),
        ("sheffield-norfolk-fc".to_string(), "div-3".to_string(), 6, "sheffield-norfolk-fc-b".to_string()),
        ("st-vincent-fc".to_string(), "div-3".to_string(), 7, "st-vincent-fc-reserves".to_string()),
        ("st-james-church-fc".to_string(), "div-3".to_string(), 8, "st-james-church-fc-reserves".to_string()),
        ("lockwood-brothers-fc".to_string(), "div-3".to_string(), 9, "lockwood-brothers-fc-second-xi".to_string()),
        ("talbot-fc".to_string(), "div-3".to_string(), 10, "talbot-fc-b".to_string()),
        ("sheffield-united-gymnastic-club".to_string(), "div-3".to_string(), 11, "sheffield-united-gymnastic-club-b".to_string()),
        ("ecclesall-college-fc".to_string(), "div-3".to_string(), 12, "ecclesall-college-fc-development".to_string()),
        ("sheffield-grammar-school-fc".to_string(), "div-3".to_string(), 13, "sheffield-grammar-school-fc-development".to_string()),
        ("surrey-catholic-club".to_string(), "div-3".to_string(), 14, "surrey-catholic-club-reserves".to_string()),
        ("attercliffe-christ-church".to_string(), "div-3".to_string(), 15, "attercliffe-christ-church-reserves".to_string()),
        ("walkey-new-connexion-fc".to_string(), "div-3".to_string(), 16, "walkey-new-connexion-fc-reserves".to_string()),

        // DIVISION 4 (16 clubs) - 1870 onwards
        ("crookes-fc".to_string(), "div-4".to_string(), 1, "crookes-fc-b".to_string()),
        ("bankers-thursday".to_string(), "div-4".to_string(), 2, "bankers-thursday-second-xi".to_string()),
        ("alliance-fc".to_string(), "div-4".to_string(), 3, "alliance-fc-b".to_string()),
        ("perseverance-fc".to_string(), "div-4".to_string(), 4, "perseverance-fc-b".to_string()),
        ("gleadless-fc".to_string(), "div-4".to_string(), 5, "gleadless-fc-b".to_string()),
        ("engineers-fc".to_string(), "div-4".to_string(), 6, "engineers-fc-second-xi".to_string()),
        ("attercliffe-zion-fc".to_string(), "div-4".to_string(), 7, "attercliffe-zion-fc-reserves".to_string()),
        ("grimesthorpe-fc".to_string(), "div-4".to_string(), 8, "grimesthorpe-fc-juniors".to_string()),
        ("exchange-brewery-fc".to_string(), "div-4".to_string(), 9, "exchange-brewery-fc-second-xi".to_string()),
        ("all-saints-night-school-fc".to_string(), "div-4".to_string(), 10, "all-saints-night-school-fc-development".to_string()),
        ("millhouses-fc".to_string(), "div-4".to_string(), 11, "millhouses-fc-b".to_string()),
        ("albion-fc".to_string(), "div-4".to_string(), 12, "albion-fc-b".to_string()),
        ("pye-bank-fc".to_string(), "div-4".to_string(), 13, "pye-bank-fc-second-xi".to_string()),
        ("brightside-fc".to_string(), "div-4".to_string(), 14, "brightside-fc-b".to_string()),
        ("norfolk-works-fc".to_string(), "div-4".to_string(), 15, "norfolk-works-fc-second-xi".to_string()),
        ("eldon-st-jude-fc".to_string(), "div-4".to_string(), 16, "eldon-st-jude-fc-reserves".to_string()),

        // DIVISION 5A (16 clubs) - 1872 onwards
        ("garden-street-fc".to_string(), "div-5a".to_string(), 1, "garden-street-fc-b".to_string()),
        ("sharrow-rangers-fc".to_string(), "div-5a".to_string(), 2, "sharrow-rangers-fc-b".to_string()),
        ("endcliffe-fc".to_string(), "div-5a".to_string(), 3, "endcliffe-fc-b".to_string()),
        ("owlerton-fc".to_string(), "div-5a".to_string(), 4, "owlerton-fc-b".to_string()),
        ("ecclesfield-fc".to_string(), "div-5a".to_string(), 5, "ecclesfield-fc-b".to_string()),
        ("philadelphia-fc".to_string(), "div-5a".to_string(), 6, "philadelphia-fc-b".to_string()),
        ("artillery-hallamshire-fc".to_string(), "div-5a".to_string(), 7, "artillery-hallamshire-fc-reserves".to_string()),
        ("intake-fc".to_string(), "div-5a".to_string(), 8, "intake-fc-b".to_string()),
        ("cherrytree-fc".to_string(), "div-5a".to_string(), 9, "cherrytree-fc-b".to_string()),
        ("victoria-burngreave-fc".to_string(), "div-5a".to_string(), 10, "victoria-burngreave-fc-juniors".to_string()),
        ("west-end-fc".to_string(), "div-5a".to_string(), 11, "west-end-fc-juniors".to_string()),
        ("bury-co-fc".to_string(), "div-5a".to_string(), 12, "bury-co-fc-second-xi".to_string()),
        ("beadshaw-baltic-fc".to_string(), "div-5a".to_string(), 13, "beadshaw-baltic-fc-second-xi".to_string()),
        ("oughtibridge-fc".to_string(), "div-5a".to_string(), 14, "oughtibridge-fc-b".to_string()),
        ("weston-fc".to_string(), "div-5a".to_string(), 15, "weston-fc-b".to_string()),
        ("wardsend-steel-works-fc".to_string(), "div-5a".to_string(), 16, "wardsend-steel-works-fc-second-xi".to_string()),

        // DIVISION 5B (16 clubs) - 1872 onwards
        ("owlerton-reform-fc".to_string(), "div-5b".to_string(), 1, "owlerton-reform-fc-reserves".to_string()),
        ("roebuck-fc".to_string(), "div-5b".to_string(), 2, "roebuck-fc-juniors".to_string()),
        ("tennant-brothers-fc".to_string(), "div-5b".to_string(), 3, "tennant-brothers-fc-second-xi".to_string()),
        ("crosspool-rangers-fc".to_string(), "div-5b".to_string(), 4, "crosspool-rangers-fc-b".to_string()),
        ("clifford-fc".to_string(), "div-5b".to_string(), 5, "clifford-fc-b".to_string()),
        ("grange-fc".to_string(), "div-5b".to_string(), 6, "grange-fc-b".to_string()),
        ("pitsmoor-coal-company-fc".to_string(), "div-5b".to_string(), 7, "pitsmoor-coal-company-fc-second-xi".to_string()),
        ("boston-street-fc".to_string(), "div-5b".to_string(), 8, "boston-street-fc-b".to_string()),
        ("birley-fc".to_string(), "div-5b".to_string(), 9, "birley-fc-b".to_string()),
        ("sherrington-fc".to_string(), "div-5b".to_string(), 10, "sherrington-fc-b".to_string()),
        ("105th-regiment".to_string(), "div-5b".to_string(), 11, "105th-regiment-reserves".to_string()),
        ("carnforth-fc".to_string(), "div-5b".to_string(), 12, "carnforth-fc-b".to_string()),
        ("hollinsend-fc".to_string(), "div-5b".to_string(), 13, "hollinsend-fc-b".to_string()),
        ("providence-fc".to_string(), "div-5b".to_string(), 14, "providence-fc-b".to_string()),
        ("handsworth-fc".to_string(), "div-5b".to_string(), 15, "handsworth-fc-b".to_string()),
        ("woodseats-fc".to_string(), "div-5b".to_string(), 16, "woodseats-fc-juniors".to_string()),

        // DIVISION 6A - WEST (16 clubs) - 1874 onwards
        ("oak-street-fc".to_string(), "div-6a".to_string(), 1, "oak-street-fc-b".to_string()),
        ("atlas-fc".to_string(), "div-6a".to_string(), 2, "atlas-fc-second-xi".to_string()),
        ("ecclesall-fc".to_string(), "div-6a".to_string(), 3, "ecclesall-fc-b".to_string()),
        ("nether-fc".to_string(), "div-6a".to_string(), 4, "nether-fc-b".to_string()),
        ("broomfield-fc".to_string(), "div-6a".to_string(), 5, "broomfield-fc-b".to_string()),
        ("st-mark-fc".to_string(), "div-6a".to_string(), 6, "st-mark-fc-reserves".to_string()),
        ("tabernacle-fc".to_string(), "div-6a".to_string(), 7, "tabernacle-fc-reserves".to_string()),
        ("st-michael-angels-fc".to_string(), "div-6a".to_string(), 8, "st-michael-angels-fc-reserves".to_string()),
        ("kenwood-fc".to_string(), "div-6a".to_string(), 9, "kenwood-fc-b".to_string()),
        ("port-mahon-fc".to_string(), "div-6a".to_string(), 10, "port-mahon-fc-b".to_string()),
        ("st-luke-fc".to_string(), "div-6a".to_string(), 11, "st-luke-fc-reserves".to_string()),
        ("heeley-victoria-fc".to_string(), "div-6a".to_string(), 12, "heeley-victoria-fc-juniors".to_string()),
        ("shrewsbury-road-fc".to_string(), "div-6a".to_string(), 13, "shrewsbury-road-fc-b".to_string()),
        ("park-united-fc".to_string(), "div-6a".to_string(), 14, "park-united-fc-b".to_string()),
        ("white-star-fc".to_string(), "div-6a".to_string(), 15, "white-star-fc-b".to_string()),
        ("good-intent-fc".to_string(), "div-6a".to_string(), 16, "good-intent-fc-b".to_string()),

        // DIVISION 6B - EAST (16 clubs)
        ("mount-tabor-fc".to_string(), "div-6b".to_string(), 1, "mount-tabor-fc-reserves".to_string()),
        ("st-jude-fc".to_string(), "div-6b".to_string(), 2, "st-jude-fc-reserves".to_string()),
        ("wingfield-rowbotham-fc".to_string(), "div-6b".to_string(), 3, "wingfield-rowbotham-fc-second-xi".to_string()),
        ("young-broomhall-fc".to_string(), "div-6b".to_string(), 4, "young-broomhall-fc-b".to_string()),
        ("regents-works-fc".to_string(), "div-6b".to_string(), 5, "regents-works-fc-second-xi".to_string()),
        ("lo-good-templars-fc".to_string(), "div-6b".to_string(), 6, "lo-good-templars-fc-reserves".to_string()),
        ("brookes-crookes-fc".to_string(), "div-6b".to_string(), 7, "brookes-crookes-fc-second-xi".to_string()),
        ("polar-star-fc".to_string(), "div-6b".to_string(), 8, "polar-star-fc-b".to_string()),
        ("norton-mount-view-fc".to_string(), "div-6b".to_string(), 9, "norton-mount-view-fc-reserves".to_string()),
        ("mill-sands-fc".to_string(), "div-6b".to_string(), 10, "mill-sands-fc-second-xi".to_string()),
        ("fenton-brothers-fc".to_string(), "div-6b".to_string(), 11, "fenton-brothers-fc-second-xi".to_string()),
        ("sir-john-brown-fc".to_string(), "div-6b".to_string(), 12, "sir-john-brown-fc-second-xi".to_string()),
        ("st-silas-fc".to_string(), "div-6b".to_string(), 13, "st-silas-fc-reserves".to_string()),
        ("ecclesall-church-fc".to_string(), "div-6b".to_string(), 14, "ecclesall-church-fc-reserves".to_string()),
        ("firth-fc".to_string(), "div-6b".to_string(), 15, "firth-fc-second-xi".to_string()),
        ("owlerton-united-fc".to_string(), "div-6b".to_string(), 16, "owlerton-united-fc-b".to_string()),

        // DIVISION 6C - NORTH (16 clubs)
        ("atkin-brothers-fc".to_string(), "div-6c".to_string(), 1, "atkin-brothers-fc-second-xi".to_string()),
        ("unitarian-fc".to_string(), "div-6c".to_string(), 2, "unitarian-fc-reserves".to_string()),
        ("deep-pits-fc".to_string(), "div-6c".to_string(), 3, "deep-pits-fc-second-xi".to_string()),
        ("sheaf-fc".to_string(), "div-6c".to_string(), 4, "sheaf-fc-second-xi".to_string()),
        ("montague-fc".to_string(), "div-6c".to_string(), 5, "montague-fc-b".to_string()),
        ("collegiate-fc".to_string(), "div-6c".to_string(), 6, "collegiate-fc-development".to_string()),
        ("stag-home-fc".to_string(), "div-6c".to_string(), 7, "stag-home-fc-juniors".to_string()),
        ("stanley-street-fc".to_string(), "div-6c".to_string(), 8, "stanley-street-fc-b".to_string()),
        ("langsett-road-fc".to_string(), "div-6c".to_string(), 9, "langsett-road-fc-b".to_string()),
        ("manor-fc".to_string(), "div-6c".to_string(), 10, "manor-fc-b".to_string()),
        ("wheatman-smith-fc".to_string(), "div-6c".to_string(), 11, "wheatman-smith-fc-second-xi".to_string()),
        ("netherthorpe-fc".to_string(), "div-6c".to_string(), 12, "netherthorpe-fc-b".to_string()),
        ("st-peter-fc".to_string(), "div-6c".to_string(), 13, "st-peter-fc-reserves".to_string()),
        ("st-philip-fc".to_string(), "div-6c".to_string(), 14, "st-philip-fc-reserves".to_string()),
        ("dronfield-united-fc".to_string(), "div-6c".to_string(), 15, "dronfield-united-fc-b".to_string()),
        ("hope-club".to_string(), "div-6c".to_string(), 16, "hope-club-second-xi".to_string()),

        // DIVISION 6D - SOUTH (16 clubs)
        ("brown-bailey-dixon-fc".to_string(), "div-6d".to_string(), 1, "brown-bailey-dixon-fc-second-xi".to_string()),
        ("ward-payne-fc".to_string(), "div-6d".to_string(), 2, "ward-payne-fc-second-xi".to_string()),
        ("crookes-united-fc".to_string(), "div-6d".to_string(), 3, "crookes-united-fc-b".to_string()),
        ("harold-fc".to_string(), "div-6d".to_string(), 4, "harold-fc-b".to_string()),
        ("huffton-son-fc".to_string(), "div-6d".to_string(), 5, "huffton-son-fc-second-xi".to_string()),
        ("malinda-works-fc".to_string(), "div-6d".to_string(), 6, "malinda-works-fc-second-xi".to_string()),
        ("parkwood-juniors-fc".to_string(), "div-6d".to_string(), 7, "parkwood-juniors-fc-b".to_string()),
        ("aston-fc".to_string(), "div-6d".to_string(), 8, "aston-fc-b".to_string()),
        ("washington-fc".to_string(), "div-6d".to_string(), 9, "washington-fc-second-xi".to_string()),
        ("st-paul-fc".to_string(), "div-6d".to_string(), 10, "st-paul-fc-reserves".to_string()),
        ("south-view-fc".to_string(), "div-6d".to_string(), 11, "south-view-fc-b".to_string()),
        ("ecclesall-united-fc".to_string(), "div-6d".to_string(), 12, "ecclesall-united-fc-b".to_string()),
        ("msl-loco-fc".to_string(), "div-6d".to_string(), 13, "msl-loco-fc-second-xi".to_string()),
        ("oxford-wanderers-fc".to_string(), "div-6d".to_string(), 14, "oxford-wanderers-fc-b".to_string()),
        ("dronfield-free-church-fc".to_string(), "div-6d".to_string(), 15, "dronfield-free-church-fc-reserves".to_string()),
        ("atlantic-juniors-fc".to_string(), "div-6d".to_string(), 16, "atlantic-juniors-fc-b".to_string()),

        // DIVISION 7A - WEST (7 clubs)
        ("wincobank-fc".to_string(), "div-7a".to_string(), 1, "wincobank-fc-b".to_string()),
        ("alexandra-fc".to_string(), "div-7a".to_string(), 2, "alexandra-fc-b".to_string()),
        ("carbrook-united-fc".to_string(), "div-7a".to_string(), 3, "carbrook-united-fc-b".to_string()),
        ("r-sorby-son-fc".to_string(), "div-7a".to_string(), 4, "r-sorby-son-fc-second-xi".to_string()),
        ("upperthorpe-fc".to_string(), "div-7a".to_string(), 5, "upperthorpe-fc-b".to_string()),
        ("howard-street-fc".to_string(), "div-7a".to_string(), 6, "howard-street-fc-b".to_string()),
        ("otley-son-fc".to_string(), "div-7a".to_string(), 7, "otley-son-fc-second-xi".to_string()),

        // DIVISION 7B - EAST (7 clubs)
        ("hillsborough-school-fc".to_string(), "div-7b".to_string(), 1, "hillsborough-school-fc-development".to_string()),
        ("bellefield-fc".to_string(), "div-7b".to_string(), 2, "bellefield-fc-second-xi".to_string()),
        ("red-rose-fc".to_string(), "div-7b".to_string(), 3, "red-rose-fc-b".to_string()),
        ("weston-rovers-fc".to_string(), "div-7b".to_string(), 4, "weston-rovers-fc-b".to_string()),
        ("j-round-son-fc".to_string(), "div-7b".to_string(), 5, "j-round-son-fc-second-xi".to_string()),
        ("dronfield-baptist-fc".to_string(), "div-7b".to_string(), 6, "dronfield-baptist-fc-reserves".to_string()),
        ("pye-bank-free-church-fc".to_string(), "div-7b".to_string(), 7, "pye-bank-free-church-fc-reserves".to_string()),

        // DIVISION 7C - NORTH (6 clubs)
        ("dronfield-independent-fc".to_string(), "div-7c".to_string(), 1, "dronfield-independent-fc-reserves".to_string()),
        ("ebenezer-reform-fc".to_string(), "div-7c".to_string(), 2, "ebenezer-reform-fc-reserves".to_string()),
        ("clough-house-fc".to_string(), "div-7c".to_string(), 3, "clough-house-fc-b".to_string()),
        ("trinity-fc".to_string(), "div-7c".to_string(), 4, "trinity-fc-second-xi".to_string()),
        ("wostenholm-fc".to_string(), "div-7c".to_string(), 5, "wostenholm-fc-second-xi".to_string()),
        ("bee-hive-works-fc".to_string(), "div-7c".to_string(), 6, "bee-hive-works-fc-second-xi".to_string()),

        // DIVISION 7D - SOUTH (6 clubs)
        ("chester-brothers-fc".to_string(), "div-7d".to_string(), 1, "chester-brothers-fc-second-xi".to_string()),
        ("heeley-united-fc".to_string(), "div-7d".to_string(), 2, "heeley-united-fc-b".to_string()),
        ("cornish-place-fc".to_string(), "div-7d".to_string(), 3, "cornish-place-fc-second-xi".to_string()),
        ("minerva-fc".to_string(), "div-7d".to_string(), 4, "minerva-fc-second-xi".to_string()),
        ("woodhouse-fc".to_string(), "div-7d".to_string(), 5, "woodhouse-fc-b".to_string()),
        ("kenyons-works-fc".to_string(), "div-7d".to_string(), 6, "kenyons-works-fc-second-xi".to_string()),
    ]
}

/// Populate the Sheffield & Hallamshire League structure into the database
pub async fn populate_sheffield_league(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Insert all divisions
    let divisions = get_all_divisions();
    for division in divisions {
        sqlx::query(
            "INSERT OR IGNORE INTO sheffield_league_divisions (id, name, level, region)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&division.id)
        .bind(&division.name)
        .bind(division.level)
        .bind(&division.region)
        .execute(pool)
        .await?;
    }

    // Create virtual club records for reserve teams (needed for foreign key constraints)
    let assignments = get_main_league_assignments();
    for (_club_id, _division_id, _position, reserve_club_id) in &assignments {
        // Only create if not already exists
        // Convert club ID to proper title case name, preserving abbreviations and special words
        let abbreviations = vec!["fc", "xi", "b"];
        let preserve_words = vec!["Reserves", "Reserve", "Juniors", "Junior", "Second"];
        let name_parts: Vec<String> = reserve_club_id
            .replace("-", " ")
            .split_whitespace()
            .map(|word| {
                // Check for parentheses
                let has_open_paren = word.starts_with('(');
                let has_close_paren = word.ends_with(')');
                let mut clean_word = word.to_string();

                if has_open_paren {
                    clean_word = clean_word[1..].to_string();
                }
                if has_close_paren {
                    clean_word = clean_word[..clean_word.len()-1].to_string();
                }

                let word_lower = clean_word.to_lowercase();
                let mut result = String::new();

                // Keep abbreviations in uppercase
                if abbreviations.contains(&word_lower.as_str()) {
                    result = clean_word.to_uppercase();
                }
                // Preserve specific words with their proper capitalization
                else if let Some(preserved) = preserve_words.iter().find(|w| w.to_lowercase() == word_lower) {
                    result = preserved.to_string();
                }
                // Capitalize first letter of each word
                else {
                    let mut chars = clean_word.chars();
                    result = match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                    };
                }

                // Add back parentheses
                if has_open_paren {
                    result = format!("({}", result);
                }
                if has_close_paren {
                    result = format!("{})", result);
                }

                result
            })
            .collect();
        let reserve_name = name_parts.join(" ");

        sqlx::query(
            "INSERT OR IGNORE INTO sheffield_clubs (id, name, founded_year, ground_name, origin)
             VALUES (?, ?, 1867, 'Unknown', 'Reserve')"
        )
        .bind(reserve_club_id)
        .bind(reserve_name)
        .execute(pool)
        .await?;
    }

    // Insert all club assignments (main league clubs)
    for (club_id, division_id, position, _reserve_club_id) in &assignments {
        sqlx::query(
            "INSERT OR IGNORE INTO sheffield_league_clubs
             (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(format!("{}-{}", club_id, division_id)) // Unique ID
        .bind(division_id)
        .bind(club_id)
        .bind(position)
        .bind(false) // Not a reserve team
        .bind::<Option<String>>(None)
        .execute(pool)
        .await?;
    }

    // Insert reserve team assignments (mirrored structure)
    for (main_club_id, division_id, position, reserve_club_id) in &assignments {
        let reserve_div_id = format!("res-{}", division_id);
        sqlx::query(
            "INSERT OR IGNORE INTO sheffield_league_clubs
             (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(format!("{}-{}", reserve_club_id, reserve_div_id))
        .bind(&reserve_div_id)
        .bind(reserve_club_id)
        .bind(position)
        .bind(true) // Is a reserve team
        .bind(Some(main_club_id.clone()))
        .execute(pool)
        .await?;
    }

    Ok(())
}
