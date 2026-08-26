/// Local Events System: Generates Victorian-era local news events for Sheffield
///
/// Historical context:
/// - Sheffield 1858-1877: Industrial boom, steel capital of England
/// - Railway expansion connecting Yorkshire
/// - Growing urban population
/// - Victorian social structure and entertainment
/// - Local governance and civic pride

use chrono::{Datelike, NaiveDate};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalEventGenerator {
    pub town: String,
    pub era_start: i64,
    pub era_end: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalEvent {
    pub event_type: LocalEventType,
    pub date: NaiveDate,
    pub headline: String,
    pub details: String,
    pub importance: EventImportance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalEventType {
    IndustrialNews,
    CivicAnnouncement,
    Transport,
    Weather,
    Entertainment,
    Crime,
    Society,
    Cooperative,  // Co-operative movement news
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventImportance {
    Minor,      // Background flavor
    Standard,   // Normal news item
    Major,      // Lead story potential
}

impl LocalEventGenerator {
    pub fn new(town: String, era_start: i64, era_end: i64) -> Self {
        Self {
            town,
            era_start,
            era_end,
        }
    }

    /// Generate 0-3 local events for a given date (weighted towards fewer events)
    pub fn generate_daily_events(&self, date: NaiveDate) -> Vec<LocalEvent> {
        let mut rng = rand::thread_rng();
        let mut events = Vec::new();

        // Weighted event count:
        // 30% chance: 0 events (quiet day)
        // 40% chance: 1 event
        // 20% chance: 2 events
        // 10% chance: 3 events (busy news day)
        let roll = rng.gen::<f32>();
        let event_count = if roll < 0.30 {
            0  // Quiet day
        } else if roll < 0.70 {
            1  // One story
        } else if roll < 0.90 {
            2  // Two stories
        } else {
            3  // Busy day
        };

        if event_count == 0 {
            return events;  // No news today
        }

        // Ensure variety by tracking types used
        let mut used_types = Vec::new();

        for _ in 0..event_count {
            let event_type = self.select_event_type(date, &used_types);
            used_types.push(event_type.clone());

            let event = self.generate_event(event_type, date);
            events.push(event);
        }

        // Sort by importance (major events first)
        events.sort_by(|a, b| {
            let a_val = match a.importance {
                EventImportance::Major => 2,
                EventImportance::Standard => 1,
                EventImportance::Minor => 0,
            };
            let b_val = match b.importance {
                EventImportance::Major => 2,
                EventImportance::Standard => 1,
                EventImportance::Minor => 0,
            };
            b_val.cmp(&a_val)
        });

        events
    }

    fn select_event_type(&self, date: NaiveDate, used_types: &[LocalEventType]) -> LocalEventType {
        let mut rng = rand::thread_rng();

        // Weight probabilities based on day of week and season
        let day_of_week = date.weekday();
        let month = date.month();

        let types = vec![
            LocalEventType::IndustrialNews,
            LocalEventType::CivicAnnouncement,
            LocalEventType::Transport,
            LocalEventType::Weather,
            LocalEventType::Entertainment,
            LocalEventType::Crime,
            LocalEventType::Society,
            LocalEventType::Cooperative,
        ];

        // Filter out already used types for variety
        let available_types: Vec<_> = types
            .into_iter()
            .filter(|t| !used_types.iter().any(|used| self.same_type(t, used)))
            .collect();

        if available_types.is_empty() {
            return LocalEventType::IndustrialNews;
        }

        // Pick random available type
        available_types[rng.gen_range(0..available_types.len())].clone()
    }

    fn same_type(&self, a: &LocalEventType, b: &LocalEventType) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    fn generate_event(&self, event_type: LocalEventType, date: NaiveDate) -> LocalEvent {
        match event_type {
            LocalEventType::IndustrialNews => self.generate_industrial_event(date),
            LocalEventType::CivicAnnouncement => self.generate_civic_event(date),
            LocalEventType::Transport => self.generate_transport_event(date),
            LocalEventType::Weather => self.generate_weather_event(date),
            LocalEventType::Entertainment => self.generate_entertainment_event(date),
            LocalEventType::Crime => self.generate_crime_event(date),
            LocalEventType::Society => self.generate_society_event(date),
            LocalEventType::Cooperative => self.generate_cooperative_event(date),
        }
    }

    // Industrial News: Steel works, cutlery, factories
    fn generate_industrial_event(&self, date: NaiveDate) -> LocalEvent {
        let mut rng = rand::thread_rng();

        let templates = vec![
            (
                "New Steel Works Opened",
                "Messrs. {company} have opened a new steel works at {location}, employing upwards of {workers} hands",
                EventImportance::Standard,
            ),
            (
                "Expansion at Cutlery Manufactory",
                "The proprietors of {company} announce a considerable expansion of their cutlery works, with new machinery of the most modern description",
                EventImportance::Minor,
            ),
            (
                "Rolling Mill Improvements",
                "Significant improvements have been made to the rolling mill at {location}, enabling the production of steel plates of superior quality",
                EventImportance::Minor,
            ),
            (
                "Railway Carriage Works Contract",
                "{company} have secured a substantial contract for the supply of railway carriages to the {railway} Railway Company",
                EventImportance::Standard,
            ),
            (
                "Forge Amalgamation",
                "It is reported that {company} and {company2} are to amalgamate their forges, creating one of the largest establishments in the district",
                EventImportance::Major,
            ),
        ];

        let (headline, template, importance) = &templates[rng.gen_range(0..templates.len())];

        let companies = vec![
            "Brown, Bayley & Dixon", "John Brown & Co.", "Cammell & Co.",
            "Jessop & Son", "Marsh Brothers", "Hadfields", "Firth & Co.",
            "Sanderson Brothers", "Vickers & Co.", "Samuel Osborn & Co.",
        ];

        let locations = vec![
            "Attercliffe", "Brightside", "Kelham Island", "Neepsend",
            "Parkwood Springs", "Wicker", "Don Valley", "Darnall",
        ];

        let railways = vec![
            "Midland", "Great Northern", "Manchester, Sheffield & Lincolnshire",
            "Great Central", "London & North Western",
        ];

        let company = companies[rng.gen_range(0..companies.len())];
        let company2 = companies[rng.gen_range(0..companies.len())];
        let location = locations[rng.gen_range(0..locations.len())];
        let railway = railways[rng.gen_range(0..railways.len())];
        let workers = rng.gen_range(50..500);

        let details = template
            .replace("{company}", company)
            .replace("{company2}", company2)
            .replace("{location}", location)
            .replace("{railway}", railway)
            .replace("{workers}", &workers.to_string());

        LocalEvent {
            event_type: LocalEventType::IndustrialNews,
            date,
            headline: headline.to_string(),
            details,
            importance: importance.clone(),
        }
    }

    // Civic Announcements: Town council, public works
    fn generate_civic_event(&self, date: NaiveDate) -> LocalEvent {
        let mut rng = rand::thread_rng();

        let templates = vec![
            (
                "Town Council Proceedings",
                "At Monday's meeting of the Town Council, it was resolved to {action}. The decision was carried by a majority of {votes}",
                EventImportance::Standard,
            ),
            (
                "New Public Baths Proposed",
                "A proposal has been brought before the Council for the construction of public baths and wash-houses in {location}, at an estimated cost of £{cost}",
                EventImportance::Standard,
            ),
            (
                "Gas Lighting Extension",
                "The Gas Committee reports that gas lighting is to be extended to {location}, with {lamps} new lamps to be erected",
                EventImportance::Minor,
            ),
            (
                "Cemetery Improvements",
                "Work has commenced on improvements to the General Cemetery, including the planting of ornamental shrubs and the repair of pathways",
                EventImportance::Minor,
            ),
            (
                "Water Supply Enhancement",
                "The Water Company announces improvements to the reservoir at {location}, which shall ensure a more abundant supply for the growing population",
                EventImportance::Standard,
            ),
        ];

        let (headline, template, importance) = &templates[rng.gen_range(0..templates.len())];

        let actions = vec![
            "approve the construction of a new market hall",
            "extend the sewerage system to newly developed districts",
            "acquire land for public recreation grounds",
            "improve the condition of public thoroughfares",
            "establish a free library for the benefit of working men",
        ];

        let locations = vec![
            "Ecclesall", "Heeley", "Walkley", "Broomhill", "Crookes",
            "Hillsborough", "Burngreave", "Sharrow", "Nether Edge",
        ];

        let action = actions[rng.gen_range(0..actions.len())];
        let location = locations[rng.gen_range(0..locations.len())];
        let votes = rng.gen_range(12..30);
        let cost = rng.gen_range(2000..15000);
        let lamps = rng.gen_range(20..100);

        let details = template
            .replace("{action}", action)
            .replace("{location}", location)
            .replace("{votes}", &votes.to_string())
            .replace("{cost}", &cost.to_string())
            .replace("{lamps}", &lamps.to_string());

        LocalEvent {
            event_type: LocalEventType::CivicAnnouncement,
            date,
            headline: headline.to_string(),
            details,
            importance: importance.clone(),
        }
    }

    // Transport: Railways, omnibuses
    fn generate_transport_event(&self, date: NaiveDate) -> LocalEvent {
        let mut rng = rand::thread_rng();

        let templates = vec![
            (
                "New Railway Service Announced",
                "The {railway} Railway Company announces a new express service between Sheffield and {destination}, departing at {time} daily",
                EventImportance::Standard,
            ),
            (
                "Omnibus Route Extended",
                "The Sheffield Omnibus Company has extended its service to {location}, with vehicles departing hourly from the Wicker",
                EventImportance::Minor,
            ),
            (
                "Railway Station Improvements",
                "Considerable improvements are being made to {station} Station, including a new booking office and waiting rooms",
                EventImportance::Minor,
            ),
            (
                "Excursion Train Arrangements",
                "Special excursion trains will run to {destination} on Saturday next, affording an excellent opportunity for holiday-makers",
                EventImportance::Minor,
            ),
            (
                "Tramway Proposal",
                "A proposal has been made for the establishment of tramways connecting the town centre with {location} and {location2}",
                EventImportance::Major,
            ),
        ];

        let (headline, template, importance) = &templates[rng.gen_range(0..templates.len())];

        let railways = vec![
            "Midland", "Great Northern", "Manchester, Sheffield & Lincolnshire",
        ];

        let destinations = vec![
            "London", "Manchester", "Leeds", "Derby", "Nottingham",
            "Birmingham", "York", "Lincoln", "Chesterfield",
        ];

        let locations = vec![
            "Ecclesall", "Heeley", "Walkley", "Broomhill", "Hillsborough",
            "Burngreave", "Sharrow", "Attercliffe",
        ];

        let stations = vec!["Victoria", "Midland", "Wicker"];

        let times = vec!["7:30 a.m.", "9:15 a.m.", "11:00 a.m.", "2:30 p.m."];

        let railway = railways[rng.gen_range(0..railways.len())];
        let destination = destinations[rng.gen_range(0..destinations.len())];
        let location = locations[rng.gen_range(0..locations.len())];
        let location2 = locations[rng.gen_range(0..locations.len())];
        let station = stations[rng.gen_range(0..stations.len())];
        let time = times[rng.gen_range(0..times.len())];

        let details = template
            .replace("{railway}", railway)
            .replace("{destination}", destination)
            .replace("{location}", location)
            .replace("{location2}", location2)
            .replace("{station}", station)
            .replace("{time}", time);

        LocalEvent {
            event_type: LocalEventType::Transport,
            date,
            headline: headline.to_string(),
            details,
            importance: importance.clone(),
        }
    }

    // Weather: Unusual conditions
    fn generate_weather_event(&self, date: NaiveDate) -> LocalEvent {
        let mut rng = rand::thread_rng();
        let month = date.month();

        let templates = if month >= 11 || month <= 2 {
            // Winter weather
            vec![
                (
                    "Severe Frost",
                    "A severe frost has prevailed throughout the district, with ice forming to a considerable thickness on standing water",
                    EventImportance::Minor,
                ),
                (
                    "Heavy Snowfall",
                    "Heavy snow fell yesterday, impeding traffic and rendering many thoroughfares well-nigh impassable",
                    EventImportance::Standard,
                ),
                (
                    "Fog Disrupts Commerce",
                    "Dense fog enveloped the town yesterday, causing considerable disruption to railway services and commercial activities",
                    EventImportance::Minor,
                ),
            ]
        } else {
            // Spring/Summer/Autumn weather
            vec![
                (
                    "Thunderstorm Damages Property",
                    "A violent thunderstorm passed over the district yesterday evening, with lightning striking {location} and causing damage to several buildings",
                    EventImportance::Standard,
                ),
                (
                    "Unseasonable Heat",
                    "The thermometer reached {temp} degrees in the shade yesterday, a most unseasonable temperature for this time of year",
                    EventImportance::Minor,
                ),
                (
                    "Heavy Rainfall Causes Flooding",
                    "Torrential rain yesterday caused the River Don to overflow its banks in several places, flooding low-lying areas",
                    EventImportance::Standard,
                ),
            ]
        };

        let (headline, template, importance) = &templates[rng.gen_range(0..templates.len())];

        let locations = vec![
            "Attercliffe", "Brightside", "Kelham Island", "the Wicker",
        ];

        let location = locations[rng.gen_range(0..locations.len())];
        let temp = rng.gen_range(75..90);

        let details = template
            .replace("{location}", location)
            .replace("{temp}", &temp.to_string());

        LocalEvent {
            event_type: LocalEventType::Weather,
            date,
            headline: headline.to_string(),
            details,
            importance: importance.clone(),
        }
    }

    // Entertainment: Theatre, concerts, exhibitions
    fn generate_entertainment_event(&self, date: NaiveDate) -> LocalEvent {
        let mut rng = rand::thread_rng();

        let templates = vec![
            (
                "Theatre Royal Performance",
                "Mr. {performer} will appear at the Theatre Royal this week in {production}, a performance which has received the highest commendation in London",
                EventImportance::Minor,
            ),
            (
                "Concert at Music Hall",
                "A grand concert will be given at the Music Hall on {day} evening, featuring {performer} and other eminent artistes",
                EventImportance::Minor,
            ),
            (
                "Art Exhibition Opens",
                "An exhibition of paintings by local artists has opened at the Mechanics' Institute, displaying works of considerable merit",
                EventImportance::Minor,
            ),
            (
                "Visiting Circus Arrives",
                "Mr. {performer}'s Celebrated Circus has arrived in the town and will give performances throughout the week, with horses, acrobats, and clowns",
                EventImportance::Standard,
            ),
            (
                "Public Lecture Announced",
                "Professor {performer} will deliver a lecture on {topic} at the Cutlers' Hall on {day} evening, admission {price}",
                EventImportance::Minor,
            ),
        ];

        let (headline, template, importance) = &templates[rng.gen_range(0..templates.len())];

        let performers = vec![
            "Charles Kean", "Samuel Phelps", "Henry Irving", "Madame Tussaud",
            "Charles Dickens", "Sanger", "Pablo Fanque", "Professor Anderson",
        ];

        let productions = vec![
            "Hamlet", "Macbeth", "The Merchant of Venice", "A Midsummer Night's Dream",
            "Richard III", "Romeo and Juliet",
        ];

        let topics = vec![
            "The Steam Engine and Modern Industry",
            "Astronomy and the Celestial Bodies",
            "The History of Sheffield Cutlery",
            "Chemistry and Its Practical Applications",
            "Natural History of the British Isles",
        ];

        let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

        let prices = vec!["6d", "1s", "1s 6d", "2s"];

        let performer = performers[rng.gen_range(0..performers.len())];
        let production = productions[rng.gen_range(0..productions.len())];
        let topic = topics[rng.gen_range(0..topics.len())];
        let day = days[rng.gen_range(0..days.len())];
        let price = prices[rng.gen_range(0..prices.len())];

        let details = template
            .replace("{performer}", performer)
            .replace("{production}", production)
            .replace("{topic}", topic)
            .replace("{day}", day)
            .replace("{price}", price);

        LocalEvent {
            event_type: LocalEventType::Entertainment,
            date,
            headline: headline.to_string(),
            details,
            importance: importance.clone(),
        }
    }

    // Crime: Court proceedings
    fn generate_crime_event(&self, date: NaiveDate) -> LocalEvent {
        let mut rng = rand::thread_rng();

        let templates = vec![
            (
                "Magistrates' Court Proceedings",
                "{name}, of {location}, was charged with being drunk and disorderly. The defendant was fined {fine} or {days} days imprisonment",
                EventImportance::Minor,
            ),
            (
                "Theft at Market",
                "{name} appeared before the magistrates charged with stealing {item} valued at {value} from the market. The case was remanded",
                EventImportance::Minor,
            ),
            (
                "Assault Case Heard",
                "{name} was charged with assault upon {name2} following a dispute at a public house. The defendant was bound over to keep the peace",
                EventImportance::Minor,
            ),
            (
                "Burglary at Warehouse",
                "A burglary was committed at the warehouse of Messrs. {company} in {location}. Property valued at £{value} was taken. The police are investigating",
                EventImportance::Standard,
            ),
        ];

        let (headline, template, importance) = &templates[rng.gen_range(0..templates.len())];

        let names = vec![
            "John Smith", "William Brown", "Thomas Jones", "George Wilson",
            "James Taylor", "Robert Davies", "Charles Evans", "Henry Wright",
        ];

        let locations = vec![
            "Trippet Lane", "High Street", "West Bar", "Haymarket",
            "Waingate", "Lady's Bridge", "Castle Street",
        ];

        let items = vec![
            "a quantity of cutlery", "several loaves of bread", "a joint of meat",
            "a pocket watch", "wearing apparel",
        ];

        let companies = vec![
            "Brown & Co.", "Jessop & Son", "Marsh Brothers", "Hadfields",
        ];

        let name = names[rng.gen_range(0..names.len())];
        let name2 = names[rng.gen_range(0..names.len())];
        let location = locations[rng.gen_range(0..locations.len())];
        let item = items[rng.gen_range(0..items.len())];
        let company = companies[rng.gen_range(0..companies.len())];
        let fine = format!("{}s", rng.gen_range(5..40));
        let days = rng.gen_range(7..28);
        let value = format!("{}s {}d", rng.gen_range(2..15), rng.gen_range(0..12));

        let details = template
            .replace("{name}", name)
            .replace("{name2}", name2)
            .replace("{location}", location)
            .replace("{item}", item)
            .replace("{company}", company)
            .replace("{fine}", &fine)
            .replace("{days}", &days.to_string())
            .replace("{value}", &value);

        LocalEvent {
            event_type: LocalEventType::Crime,
            date,
            headline: headline.to_string(),
            details,
            importance: importance.clone(),
        }
    }

    // Society: Social gatherings, charitable events
    fn generate_society_event(&self, date: NaiveDate) -> LocalEvent {
        let mut rng = rand::thread_rng();

        let templates = vec![
            (
                "Charitable Bazaar Success",
                "A bazaar in aid of {charity} was held yesterday at {location}, realising the handsome sum of £{amount}. The event was well attended by the gentry and principal inhabitants",
                EventImportance::Minor,
            ),
            (
                "Annual Dinner Held",
                "The annual dinner of the {organization} was held on {day} evening at the Cutlers' Hall, with upwards of {guests} gentlemen present",
                EventImportance::Minor,
            ),
            (
                "Wedding Announcement",
                "A fashionable wedding took place yesterday at {church}, when {name} Esq., of {location}, was united to Miss {name2}, daughter of {father} Esq.",
                EventImportance::Minor,
            ),
            (
                "Public Subscription Opened",
                "A public subscription has been opened for {cause}, with the Mayor heading the list with a donation of £{amount}",
                EventImportance::Standard,
            ),
            (
                "Working Men's Club Founded",
                "A Working Men's Club and Institute has been founded in {location}, providing educational and recreational facilities for artisans and labourers",
                EventImportance::Standard,
            ),
        ];

        let (headline, template, importance) = &templates[rng.gen_range(0..templates.len())];

        let charities = vec![
            "the Infirmary", "the Orphanage", "the Poor Relief Fund",
            "the Soup Kitchen", "the Ragged School",
        ];

        let causes = vec![
            "the relief of distressed cutlers",
            "the establishment of a Working Men's College",
            "the provision of coal for the poor during winter",
            "the victims of the recent mining disaster",
        ];

        let organizations = vec![
            "Sheffield and Hallamshire Cricket Club",
            "Cutlers' Company",
            "Chamber of Commerce",
            "Master Cutlers' Company",
            "Mechanics' Institute",
        ];

        let churches = vec![
            "Sheffield Parish Church", "St. George's Church", "St. Paul's Church",
        ];

        let names = vec![
            "Charles", "William", "George", "Henry", "Frederick", "Edward",
        ];

        let female_names = vec![
            "Elizabeth", "Mary", "Sarah", "Emma", "Charlotte", "Harriet",
        ];

        let locations = vec![
            "Broomhall", "Ranmoor", "Ecclesall", "Endcliffe", "Nether Edge",
        ];

        let charity = charities[rng.gen_range(0..charities.len())];
        let cause = causes[rng.gen_range(0..causes.len())];
        let organization = organizations[rng.gen_range(0..organizations.len())];
        let church = churches[rng.gen_range(0..churches.len())];
        let location = locations[rng.gen_range(0..locations.len())];
        let name = names[rng.gen_range(0..names.len())];
        let name2 = female_names[rng.gen_range(0..female_names.len())];
        let father = names[rng.gen_range(0..names.len())];
        let amount = rng.gen_range(50..500);
        let guests = rng.gen_range(80..200);
        let day = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"][rng.gen_range(0..5)];

        let details = template
            .replace("{charity}", charity)
            .replace("{cause}", cause)
            .replace("{organization}", organization)
            .replace("{church}", church)
            .replace("{location}", location)
            .replace("{name}", name)
            .replace("{name2}", name2)
            .replace("{father}", father)
            .replace("{amount}", &amount.to_string())
            .replace("{guests}", &guests.to_string())
            .replace("{day}", day);

        LocalEvent {
            event_type: LocalEventType::Society,
            date,
            headline: headline.to_string(),
            details,
            importance: importance.clone(),
        }
    }

    // Co-operative Movement: News from the Brightside and Carbrook Co-operative
    fn generate_cooperative_event(&self, date: NaiveDate) -> LocalEvent {
        let mut rng = rand::thread_rng();

        let templates = vec![
            (
                "Co-operative Society Lantern Lecture",
                "A lantern lecture on 'The Principles of Fair Trade' will be delivered at the Co-operative Hall on Saturday evening, admission free to members",
                EventImportance::Minor,
            ),
            (
                "Educational Classes for Working Men",
                "The Co-operative Society announces the commencement of educational classes covering arithmetic, reading, and the principles of mutual aid",
                EventImportance::Standard,
            ),
            (
                "Co-operative Social Gathering",
                "A social evening for members of the Co-operative will be held at the Brightside Hall, with refreshments and entertainment provided by the Society",
                EventImportance::Minor,
            ),
            (
                "New Co-operative Branch Opened",
                "The Co-operative Society has opened a new branch shop in {location}, extending its service and principles of fair trade to a wider district",
                EventImportance::Standard,
            ),
            (
                "Annual Co-operative Meeting",
                "The Annual General Meeting of the Brightside and Carbrook Co-operative Society will convene on Monday evening to review the year's considerable progress",
                EventImportance::Standard,
            ),
        ];

        let (headline, template, importance) = &templates[rng.gen_range(0..templates.len())];

        let locations = vec![
            "Carbrook", "Attercliffe", "Darnall", "Brightside", "Burngreave",
        ];

        let location = locations[rng.gen_range(0..locations.len())];

        let details = template.replace("{location}", location);

        LocalEvent {
            event_type: LocalEventType::Cooperative,
            date,
            headline: headline.to_string(),
            details,
            importance: importance.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_generation() {
        let generator = LocalEventGenerator::new("Sheffield".to_string(), 1858, 1877);
        let date = NaiveDate::from_ymd_opt(1867, 10, 15).unwrap();

        // Test multiple days to verify distribution
        let mut total_events = 0;
        let mut days_tested = 0;

        for day in 0..100 {
            let test_date = date + chrono::Duration::days(day);
            let events = generator.generate_daily_events(test_date);
            total_events += events.len();
            days_tested += 1;

            // Each day should have 0-3 events
            assert!(events.len() <= 3, "Too many events on a single day");
        }

        // Average should be around 1-1.5 events per day (given our 30/40/20/10 distribution)
        let average = total_events as f32 / days_tested as f32;
        assert!(average >= 0.8 && average <= 2.0, "Average events per day out of expected range: {}", average);
    }

    #[test]
    fn test_event_variety() {
        let generator = LocalEventGenerator::new("Sheffield".to_string(), 1858, 1877);
        let date = NaiveDate::from_ymd_opt(1867, 10, 15).unwrap();

        // Generate events for multiple days to test variety
        for day in 0..50 {
            let test_date = date + chrono::Duration::days(day);
            let events = generator.generate_daily_events(test_date);

            // Should not have duplicate event types on same day
            let mut types = Vec::new();
            for event in &events {
                let type_str = format!("{:?}", event.event_type);
                assert!(!types.contains(&type_str), "Duplicate event type on same day: {}", type_str);
                types.push(type_str);
            }
        }
    }

    #[test]
    fn test_quiet_days_occur() {
        let generator = LocalEventGenerator::new("Sheffield".to_string(), 1858, 1877);
        let date = NaiveDate::from_ymd_opt(1867, 10, 15).unwrap();

        let mut quiet_days = 0;

        // Test 100 days
        for day in 0..100 {
            let test_date = date + chrono::Duration::days(day);
            let events = generator.generate_daily_events(test_date);

            if events.is_empty() {
                quiet_days += 1;
            }
        }

        // Should have some quiet days (roughly 30% based on our distribution)
        assert!(quiet_days > 10, "Not enough quiet days: {}", quiet_days);
        assert!(quiet_days < 50, "Too many quiet days: {}", quiet_days);
    }
}
