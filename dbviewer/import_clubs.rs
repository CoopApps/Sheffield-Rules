use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    eprintln!("=== CREATING SHEFFIELD_CLUBS TABLE ===\n");

    // Drop and create table
    conn.execute("DROP TABLE IF EXISTS sheffield_clubs", [])?;
    conn.execute(
        "CREATE TABLE sheffield_clubs (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            founded_year INTEGER NOT NULL,
            ground TEXT,
            origin TEXT,
            city TEXT,
            region TEXT
        )",
        [],
    )?;

    // All club data from clubs.rs with postcodes filled in
    let clubs = vec![
        // 1857-1858
        ("sheffield-fc", "Sheffield FC", 1857, "East Bank", "played at East Bank", None, Some("S2")),
        // 1860
        ("hallam-fc", "Hallam FC", 1860, "Sandygate", "play at Sandygate", None, Some("S10")),
        // 1861
        ("norfolk-fc", "Norfolk FC", 1861, "Norfolk Park", "played at Norfolk Park", Some("Norfolk Park"), Some("S2")),
        ("cemetery-road-church-fc", "Cemetery Road Church FC", 1861, "Hunters Bar", "Oldest church club, played at Hunters Bar", None, Some("S11")),
        ("york-fc", "York FC", 1861, "Endcliffe Cricket Ground", "From York Hotel, Broomhill, played at Endcliffe Cricket Ground, N Creswick was President", Some("Broomhill"), Some("S11")),
        ("norton-fc", "Norton FC", 1861, "Oaks Park", "played at Oaks Park, Norton", Some("Norton"), Some("S8")),
        ("pitsmoor-fc", "Pitsmoor FC", 1861, "Pitsmoor CC", "played at Pitsmoor CC, now SUFC Academy", Some("Pitsmoor"), Some("S4")),
        // 1862
        ("fir-vale-fc", "Fir Vale FC", 1862, "Pitsmoor CC", "played at Pitsmoor CC, now SUFC Academy", Some("Pitsmoor"), Some("S4")),
        ("heeley-christ-church-fc", "Heeley Christ Church FC", 1862, "Meersbrook Park", "played at Meersbrook Park", Some("Meersbrook"), Some("S8")),
        ("mackenzie-fc", "Mackenzie FC", 1862, "Myrtle Road, Heeley", "played at Myrtle Road, Heeley", Some("Heeley"), Some("S2")),
        ("milton-fc", "Milton FC", 1862, "Cremorne Gardens, London Road", "played at Cremorne Gardens, London Road", None, Some("S10")),
        ("howard-hill-steel-bank-fc", "Howard Hill Steel Bank FC", 1862, "Howard Hotel, Howard Road", "Met in Howard Hotel, Howard Road", None, Some("S6")),
        ("ranmoor-fc", "Ranmoor FC", 1862, "Ranmoor", "played in Ranmoor", Some("Ranmoor"), Some("S10")),
        ("st-george-fc", "St George FC", 1862, "Broad Lane", "From St George's Church, Broad Lane.", None, Some("S1")),
        ("st-stephen-fc", "St Stephen FC", 1862, "Crookes", "From St Stephen's Church Fawcett Road Netherthorpe. Played at Crookes.", Some("Netherthorpe"), Some("S10")),
        ("united-norfolk-fc", "United Norfolk FC", 1862, "Unknown", "Origins uncertain", None, Some("S2")),
        // 1863
        ("crabtree-fc", "Crabtree FC", 1863, "Fir Vale area", "Likely from Fir Vale area", Some("Fir Vale"), Some("S4")),
        ("broomhall-fc", "Broomhall FC", 1863, "Ecclesall Road", "played at Ecclesall Road", Some("Ecclesall"), Some("S11")),
        ("tudor-fc", "Tudor FC", 1863, "Unknown", "Other", None, Some("S3")),
        ("w-h-hutchinson-fc", "W & H Hutchinson's FC", 1863, "Unknown", "Works", None, Some("S3")),
        ("hemsworth-fc", "Hemsworth FC", 1863, "Unknown", "Other", None, Some("S5")),
        // 1865
        ("united-mechanics", "United Mechanics", 1865, "Norfolk Park", "played at Norfolk Park", Some("Norfolk Park"), Some("S2")),
        // 1866
        ("garrick-fc", "Garrick FC", 1866, "East Bank", "played at East Bank", None, Some("S2")),
        ("wellington-fc", "Wellington FC", 1866, "Hounsfield Park", "played at Hounsfield Park near Bramall Lane", None, Some("S6")),
        ("loxley-fc", "Loxley FC", 1866, "The Rodney Inn, Loxley", "Met at The Rodney Inn, Loxley", Some("Loxley"), Some("S6")),
        // 1867
        ("wednesday-fc", "Wednesday FC", 1867, "Highfields, now Hillsborough", "played at Highfields, now Hillsborough", Some("Hillsborough"), Some("S2")),
        ("exchange-fc", "Exchange FC", 1867, "Hallam's Farm, now Hyde Park Flats", "played at Hallam's Farm, now Hyde Park Flats", None, Some("S2")),
        ("dore-fc", "Dore FC", 1867, "The Devonshire Arms, Dore", "Met at The Devonshire Arms, Dore", Some("Dore"), Some("S17")),
        ("tapton-fc", "Tapton FC", 1867, "Tapton Hall", "Based at Tapton Hall", None, Some("S10")),
        // 1868
        ("dronfield-fc", "Dronfield FC", 1868, "Bagley's Field, Dronfield", "played at Bagley's Field, Dronfield", None, Some("S18")),
        ("brincliffe-fc", "Brincliffe FC", 1868, "Cherry Tree Farm", "played at Cherry Tree Farm, near Union pub", None, Some("S11")),
        ("hanover-united-fc", "Hanover United FC", 1868, "Crookes", "played at Crookes", Some("Crookes"), Some("S10")),
        ("stannington-fc", "Stannington FC", 1868, "Unknown", "unknown ground location", None, Some("S6")),
        ("redhill-fc", "Redhill FC", 1868, "Winter Street, near Weston Park", "played at Winter Street, near Weston Park", None, Some("S6")),
        // 1869
        ("parkwood-springs-fc", "Parkwood Springs FC", 1869, "Parkwood Springs Recreation Ground", "played at Parkwood Springs Recreation Ground", None, Some("S3")),
        ("oxford-fc", "Oxford FC", 1869, "Ecclesall Road", "played at Ecclesall Road", Some("Ecclesall"), Some("S11")),
        ("totley-fc", "Totley FC", 1869, "Cross Scythes Inn, Totley", "played at field next to Cross Scythes Inn, Totley", Some("Totley"), Some("S17")),
        ("sheffield-norfolk-fc", "Sheffield Norfolk FC", 1869, "Unknown", "Other", None, Some("S2")),
        ("st-vincent-fc", "St Vincent's", 1869, "Queens Ground", "from Solly Street - played at Queens Ground", None, Some("S2")),
        ("st-james-church-fc", "St James Church FC", 1869, "Norton Lees Lane", "played at Norton Lees Lane", Some("Norton Lees"), Some("S8")),
        // 1870
        ("lockwood-brothers-fc", "Lockwood Brothers FC", 1870, "Hunters Bar", "played at Hunters Bar, oldest works club", None, Some("S11")),
        ("talbot-fc", "Talbot FC", 1870, "Norfolk Road", "played at Norfolk Road", None, Some("S2")),
        ("sheffield-united-gymnastic-club", "Sheffield United Gymnastic Club", 1870, "Unknown", "Other", None, Some("S1")),
        ("ecclesall-college-fc", "Ecclesall College FC", 1870, "Unknown", "School", None, Some("S11")),
        ("sheffield-grammar-school-fc", "Sheffield Grammar School FC", 1870, "Unknown", "School", None, Some("S10")),
        ("surrey-catholic-club", "Surrey Catholic Club", 1870, "The Farm, now Sheffield College", "played at The Farm, now Sheffield College", None, Some("S1")),
        ("attercliffe-christ-church", "Attercliffe (Christ Church)", 1870, "The Old Forge Ground, Shirland Lane", "played at The Old Forge Ground, Shirland Lane", None, Some("S9")),
        ("walkey-new-connexion-fc", "Walkey New Connexion FC", 1870, "Queens Ground, Hillsborough", "played at Queens Ground, Hillsborough", Some("Hillsborough"), Some("S2")),
        ("crookes-fc", "Crookes FC", 1870, "Lydgate Lane", "played at Lydgate Lane", None, Some("S10")),
        ("bankers-thursday", "Bankers / Thursday", 1870, "Hunters Bar", "played at Hunters Bar", None, Some("S11")),
        ("alliance-fc", "Alliance FC", 1870, "Norfolk Park", "played at Norfolk Park", Some("Norfolk Park"), Some("S2")),
        ("perseverance-fc", "Perseverance FC", 1870, "Norfolk Park", "played at Norfolk Park", Some("Norfolk Park"), Some("S2")),
        ("gleadless-fc", "Gleadless FC", 1870, "Charnock Hall, Gleadless", "played at Charnock Hall, Gleadless", Some("Gleadless"), Some("S12")),
        ("engineers-fc", "Engineers FC", 1870, "Endcliffe Crescent", "Played at Endcliffe Crescent", None, Some("S11")),
        // 1871
        ("attercliffe-zion-fc", "Attercliffe Zion FC", 1871, "Unknown", "from Zion Church, Attercliffe", None, Some("S9")),
        ("grimesthorpe-fc", "Grimesthorpe FC", 1871, "Victoria Hotel, Grimesthorpe", "Met at Victoria Hotel, Grimesthorpe", Some("Grimesthorpe"), Some("S4")),
        ("exchange-brewery-fc", "Exchange Brewery FC", 1871, "Fox Street, Pye Bank", "played at Fox Street, Pye Bank", None, Some("S1")),
        ("all-saints-night-school-fc", "All Saints Night School FC", 1871, "Hall Carr Lane, now Carwood Road", "Hall Carr Lane, now Carwood Road", None, Some("S4")),
        ("millhouses-fc", "Millhouses FC", 1871, "The Old Corn Mill", "possibly played at The Old Corn Mill", None, Some("S7")),
        // 1872
        ("albion-fc", "Albion FC", 1872, "Ecclesall Road", "played at Ecclesall Road", Some("Ecclesall"), Some("S11")),
        ("pye-bank-fc", "Pye Bank FC", 1872, "Fox Street, Pye Bank", "played at Fox Street, Pye Bank", None, Some("S3")),
        ("brightside-fc", "Brightside FC", 1872, "Blackburn Meadows", "played at Blackburn Meadows, Blackburn", None, Some("S9")),
        ("norfolk-works-fc", "Norfolk Works FC", 1872, "Newhall Athletic Ground", "Played at Newhall Athletic Ground", None, Some("S9")),
        ("eldon-st-jude-fc", "Eldon St Jude's FC", 1872, "Brocco Bank", "played at Brocco Bank", None, Some("S11")),
        ("garden-street-fc", "Garden Street FC", 1872, "Hollins Crog", "From Garden Street, Hollins Crog", None, Some("S4")),
        // 1873 - Major expansion
        ("sharrow-rangers-fc", "Sharrow Rangers FC", 1873, "Crescent Road, Sharrow", "played at Crescent Road, Sharrow", Some("Sharrow"), Some("S7")),
        ("endcliffe-fc", "Endcliffe FC", 1873, "Ecclesall Road", "played at Ecclesall Road", None, Some("S11")),
        ("owlerton-fc", "Owlerton FC", 1873, "Rawson's Meadow Ground, Owlerton", "played at Rawson's Meadow Ground, Owlerton", None, Some("S6")),
        ("ecclesfield-fc", "Ecclesfield FC", 1873, "Fairham's Crog", "played at Fairham's Crog", None, Some("S35")),
        ("philadelphia-fc", "Philadelphia FC", 1873, "Queens Ground, Hillsborough", "played at Queens Ground Hillsborough", Some("Hillsborough"), Some("S2")),
        ("artillery-hallamshire-fc", "Artillery & Hallamshire FC", 1873, "Endcliffe Hall", "Based at Endcliffe Hall (later just Artillery)", None, Some("S11")),
        ("intake-fc", "Intake FC", 1873, "Intake", "From Intake", Some("Intake"), Some("S12")),
        ("cherrytree-fc", "Cherrytree FC", 1873, "Cherrytree Orphanage", "Possibly from Cherrytree Orphanage", None, Some("S11")),
        ("victoria-burngreave-fc", "Victoria (Burngreave) FC", 1873, "Hall Carr Lane", "played at Hall Carr Lane, east end", None, Some("S4")),
        ("west-end-fc", "West End FC", 1873, "Hunters Bar", "played at Hunters Bar, from West End Hotel", None, Some("S11")),
        ("bury-co-fc", "Bury's & Co FC", 1873, "Regents Works", "From Regents Works, now Wicks", None, Some("S6")),
        ("beadshaw-baltic-fc", "Beadshaw's (Baltic) FC", 1873, "Baltic Works, Attercliffe", "From Baltic Works, Attercliffe", None, Some("S9")),
        ("oughtibridge-fc", "Oughtibridge FC", 1873, "Oughtibridge", "From Oughtibridge", Some("Oughtibridge"), Some("S35")),
        ("weston-fc", "Weston FC", 1873, "Weston Hall", "Possibly played at Weston Hall", None, Some("S1")),
        ("wardsend-steel-works-fc", "Wardsend Steel Works FC", 1873, "Herries Road", "From Wardsend Steel Works, Herries Road", None, Some("S5")),
        ("owlerton-reform-fc", "Owlerton Reform FC", 1873, "Borough Road, Owlerton", "From Wesleyan Reform Church on Borough Road, Owlerton", None, Some("S6")),
        ("roebuck-fc", "Roebuck FC", 1873, "East Bank", "Roebuck pub, played at East Bank", None, Some("S2")),
        ("tennant-brothers-fc", "Tennant Brothers & Co FC", 1873, "Exchange Brewery", "club from Exchange Brewery", None, Some("S1")),
        ("crosspool-rangers-fc", "Crosspool Rangers FC", 1873, "Crosspool", "From Crosspool", Some("Crosspool"), Some("S10")),
        ("clifford-fc", "Clifford FC", 1873, "Psalter Lane", "From Clifford House, Psalter Lane", None, Some("S11")),
        ("grange-fc", "Grange FC", 1873, "Intake Road", "played at Intake Road", Some("Intake"), Some("S12")),
        ("pitsmoor-coal-company-fc", "Pitsmoor Coal Company FC", 1873, "Brightside", "From Brightside", Some("Brightside"), Some("S4")),
        ("boston-street-fc", "Boston Street FC", 1873, "Heeley", "From Boston Street, played in Heeley", Some("Heeley"), Some("S9")),
        ("birley-fc", "Birley FC", 1873, "Hollinsend", "played at Hollinsend, Birley", None, Some("S12")),
        ("sherrington-fc", "Sherrington FC", 1873, "Norfolk Park", "From Sherrington Road, Sharrow, played Norfolk Park", Some("Norfolk Park"), Some("S2")),
        // 1874 - Continued expansion
        ("105th-regiment", "105th Regiment", 1874, "Unknown", "Joined FA in 1874. Played in FA Cup 1875 to 1879. Nickname was 'Light Bobs'", None, Some("S6")),
        ("carnforth-fc", "Carnforth FC", 1874, "Sharrow Vale Road", "played at Sharrow Vale Road", Some("Sharrow"), Some("S11")),
        ("hollinsend-fc", "Hollinsend FC", 1874, "Unknown", "Other", None, Some("S12")),
        ("providence-fc", "Providence FC", 1874, "Park Hill Lane", "played at Park Hill Lane", Some("Park Hill"), Some("S2")),
        ("handsworth-fc", "Handsworth FC", 1874, "Unknown", "Ground unknown", None, Some("S13")),
        ("woodseats-fc", "Woodseats FC", 1874, "Woodseats Hotel", "Met at the Woodseats Hotel, now Viraaj", Some("Woodseats"), Some("S8")),
        ("oak-street-fc", "Oak Street FC", 1874, "Heeley", "From street of same name, Heeley", Some("Heeley"), Some("S2")),
        ("atlas-fc", "Atlas FC", 1874, "East End", "formed from Atlas Works, East End", None, Some("S9")),
        ("ecclesall-fc", "Ecclesall FC", 1874, "Hunters Bar", "played at Hunters Bar", None, Some("S11")),
        ("nether-fc", "Nether FC", 1874, "Eastborne", "played at Eastborne", None, Some("S2")),
        ("broomfield-fc", "Broomfield FC", 1874, "Broomfield", "From Broomfield area of Sheffield", None, Some("S2")),
        ("st-mark-fc", "St Mark's FC", 1874, "Broomfield Road", "From St Mark's Broomfield Road", None, Some("S2")),
        ("tabernacle-fc", "Tabernacle FC", 1874, "Albert Terrace Road", "From Tabernacle Church, Albert Terrace Road", None, Some("S8")),
        ("st-michael-angels-fc", "St Michaels Angels FC", 1874, "Parkwood Springs", "From St Michael's Church, Parkwood Springs", None, Some("S3")),
        ("kenwood-fc", "Kenwood FC", 1874, "Abbeydale Road", "played at a ground off Abbeydale Road", None, Some("S7")),
        ("port-mahon-fc", "Port Mahon FC", 1874, "Port Mahon", "From Port Mahon, now Ponderosa", None, Some("S1")),
        ("st-luke-fc", "St Luke's FC", 1874, "Park", "From St Luke's, Park (behind Midland Station),", None, Some("S2")),
        ("heeley-victoria-fc", "Heeley Victoria FC", 1874, "Heeley", "From Victoria Hotel in Heeley", Some("Heeley"), Some("S2")),
        ("shrewsbury-road-fc", "Shrewsbury Road FC", 1874, "Behind Midland Station", "Road of same name behind Midland station", None, Some("S8")),
        ("park-united-fc", "Park United FC", 1874, "Park", "From Park area", None, Some("S2")),
        ("white-star-fc", "White Star FC", 1874, "Unknown", "Location origins unknown", None, Some("S1")),
        ("good-intent-fc", "Good Intent FC", 1874, "Truro Ground, Matilda Street", "Likely played at Truro Ground on Matilda Street", None, Some("S3")),
        ("mount-tabor-fc", "Mount Tabor FC", 1874, "City Centre", "Church in city centre, now demolish", None, Some("S10")),
        ("st-jude-fc", "St Jude's FC", 1874, "Cupola Street", "From St Jude's church on Cupola Street", None, Some("S9")),
        ("wingfield-rowbotham-fc", "Wingfield & Rowbotham FC", 1874, "Tenter Street", "From their works on Tenter Street", None, Some("S1")),
        ("young-broomhall-fc", "Young Broomhall FC", 1874, "Unknown", "Ground location unknown", None, Some("S10")),
        ("regents-works-fc", "Regents Works FC", 1874, "Penistone Road", "Regents Works, now Wicks on Penistone Road", Some("Penistone"), Some("S6")),
        ("lo-good-templars-fc", "L. O. Good Templars FC", 1874, "Unknown", "Origins unknown", None, Some("S1")),
        ("brookes-crookes-fc", "Brookes & Crookes FC", 1874, "Brook Lane", "From Brook Steel Works, Brook Lane", None, Some("S3")),
        ("polar-star-fc", "Polar Star FC", 1874, "Norfolk Park", "played at Norfolk Park", Some("Norfolk Park"), Some("S2")),
        ("norton-mount-view-fc", "Norton Mount View FC", 1874, "Norton Lees", "From Mount View Methodists Church, Norton Lees", Some("Norton Lees"), Some("S8")),
        ("mill-sands-fc", "Mill Sands FC", 1874, "Mill Sands Works", "From Mill Sands Works, now Vulcan House", None, Some("S9")),
        ("fenton-brothers-fc", "Fenton Brothers FC", 1874, "East Street, Park", "From their works, East Street, Park", None, Some("S2")),
        ("sir-john-brown-fc", "Sir John Brown's FC", 1874, "Osgathorpe", "played at Osgathorpe, near Earl Marshall", None, Some("S9")),
        ("st-silas-fc", "St Silas FC", 1874, "Broomhall", "From St Silas Church, Broomhall", Some("Broomhall"), Some("S2")),
        ("ecclesall-church-fc", "Ecclesall Church FC", 1874, "Ecclesall", "From Ecclesall All Saints", Some("Ecclesall"), Some("S11")),
        ("firth-fc", "Firth's FC", 1874, "East End", "From Thomas Firth & Son's, east end", None, Some("S9")),
        ("owlerton-united-fc", "Owlerton United FC", 1874, "Wadsley Bridge", "played at Wadsley Bridge", Some("Wadsley Bridge"), Some("S6")),
        ("atkin-brothers-fc", "Atkin Brothers FC", 1874, "Matilda Street", "From Truro Works, Matilda Street", None, Some("S1")),
        ("unitarian-fc", "Unitarian FC", 1874, "Norfolk Street", "From Unitarian Church, Norfolk Street", None, Some("S1")),
        ("deep-pits-fc", "Deep Pits FC", 1874, "City Road", "Near Manor Top, on City Road", Some("Manor"), Some("S2")),
        ("sheaf-fc", "Sheaf FC", 1874, "Victoria Station", "From Sheaf Works next to Victoria station", None, Some("S2")),
        ("montague-fc", "Montague FC", 1874, "Sharrow", "From Montague Street Sharrow", Some("Sharrow"), Some("S7")),
        // 1875 - Final major expansion before FA standardization
        ("collegiate-fc", "Collegiate FC", 1875, "Unknown", "(exact date unknown)", None, Some("S10")),
        ("stag-home-fc", "Stag Home FC", 1875, "Unknown", "Origins uncertain, possibly played at Kenwood", None, Some("S1")),
        ("stanley-street-fc", "Stanley Street FC", 1875, "Norfolk Park", "From Stanley Street, Wicker, played mainly at Norfolk Park", Some("Norfolk Park"), Some("S2")),
        ("langsett-road-fc", "Langsett Road FC", 1875, "Queens Ground", "Likely played at Queens Ground", None, Some("S6")),
        ("manor-fc", "Manor FC", 1875, "Manor Lane", "played at Manor Lane", Some("Manor"), Some("S2")),
        ("wheatman-smith-fc", "Wheatman & Smith's FC", 1875, "Russell Works", "From Russell Works, near Kelham Island", None, Some("S9")),
        ("netherthorpe-fc", "Netherthorpe FC", 1875, "Unknown", "Ground location uncertain", None, Some("S3")),
        ("st-peter-fc", "St Peter's FC", 1875, "Myrtle Road", "From the now Cathedral, played at Myrtle Road", None, Some("S2")),
        ("st-philip-fc", "St Philip's FC", 1875, "Netherthorpe", "From St Philip's church, Netherthorpe", None, Some("S2")),
        ("dronfield-united-fc", "Dronfield United FC", 1875, "Dronfield", "From Dronfield", None, Some("S18")),
        ("hope-club", "Hope Club", 1875, "Weston Field", "From Hope Works on Sussex Road, Played at Weston Field", Some("Hope"), Some("S1")),
        ("brown-bailey-dixon-fc", "Brown Bailey & Dixon FC", 1875, "Attercliffe", "From their Attercliffe works, Leeds Road", None, Some("S9")),
        ("ward-payne-fc", "Ward & Payne's FC", 1875, "Limbrick Works, Hillsborough", "From their Limbrick Works, Hillsborough", None, Some("S6")),
        ("crookes-united-fc", "Crookes United FC", 1875, "Lydgate Lane", "Likely played at Lydgate Lane", None, Some("S10")),
        ("harold-fc", "Harold FC", 1875, "Walkley", "Likely from Harold Street, Walkley", Some("Walkley"), Some("S1")),
        ("huffton-son-fc", "Huffton & Son FC", 1875, "West Street", "From Huffton's Works, West Street", None, Some("S1")),
        ("malinda-works-fc", "Malinda Works FC", 1875, "Malinda Street", "From Malinda Works, Malinda Street", None, Some("S9")),
        ("parkwood-juniors-fc", "Parkwood Juniors FC", 1875, "Parkwood Springs", "From Parkwood Springs", None, Some("S3")),
        ("aston-fc", "Aston FC", 1875, "Aston", "From Aston", Some("Aston"), Some("S26")),
        ("washington-fc", "Washington FC", 1875, "Washington Street", "From Wostenholm's Washington Works, Washington Street, poss played at Cobden View", None, Some("S3")),
        ("st-paul-fc", "St Paul's FC", 1875, "Peace Gardens", "From St Paul's church, now Peace Gardens", None, Some("S1")),
        ("south-view-fc", "South View FC", 1875, "Machon Bank", "From South View Road in Sharrow, played at Machon Bank", Some("Sharrow"), Some("S7")),
        ("ecclesall-united-fc", "Ecclesall United FC", 1875, "Ecclesall", "played at Ecclesall", Some("Ecclesall"), Some("S11")),
        ("msl-loco-fc", "MS&L Loco FC", 1875, "Unknown", "Ground location unknown", None, Some("S4")),
        ("oxford-wanderers-fc", "Oxford Wanderers FC", 1875, "Norfolk Park", "Poss from Oxford Street, played Norfolk Park", Some("Norfolk Park"), Some("S2")),
        ("dronfield-free-church-fc", "Dronfield Free Church FC", 1875, "Dronfield", "From Dronfield United Methodist Free Church, now Peel Centre, High Street, Dronfield", None, Some("S18")),
        ("atlantic-juniors-fc", "Atlantic (Juniors) FC", 1875, "Cobden View", "played at Cobden View", None, Some("S3")),
        ("wincobank-fc", "Wincobank FC", 1875, "Wincobank", "From Wincobank area", Some("Wincobank"), Some("S9")),
        ("alexandra-fc", "Alexandra FC", 1875, "Alexandra Road, Heeley", "From Alexandra Road, Heeley", Some("Heeley"), Some("S2")),
        ("carbrook-united-fc", "Carbrook United FC", 1875, "Carbrook", "played behind St Batholowmew's Church Carbrook", None, Some("S9")),
        ("r-sorby-son-fc", "R Sorby & Son FC", 1875, "Trafalgar Street", "From Kangaroo Works, Trafalgar Street", None, Some("S1")),
        ("upperthorpe-fc", "Upperthorpe FC", 1875, "Cobden View", "played at Cobden View", None, Some("S3")),
        ("howard-street-fc", "Howard Street FC", 1875, "Heeley", "Likely played in Heeley, from Howard Street", Some("Heeley"), Some("S2")),
        ("otley-son-fc", "Otley & Son's FC", 1875, "Shalesmoor", "From Meadow Works Shalesmoor", None, Some("S3")),
        ("hillsborough-fc", "Hillsborough / Hillsborough School FC", 1875, "Hillsborough", "played at Hillsborough", Some("Hillsborough"), Some("S6")),
        ("bellefield-fc", "Bellefield FC", 1875, "Bellefield Lane, Netherthorpe", "From Bellefield Works, Bellefield Lane, Netherthorpe", None, Some("S3")),
        ("red-rose-fc", "Red Rose FC", 1875, "Unknown", "Could be from Pitsmoor or Bramall Lane or was possibly 2 clubs", Some("Pitsmoor"), Some("S1")),
        ("weston-rovers-fc", "Weston Rovers FC", 1875, "Unknown", "Uncertain origins", None, Some("S1")),
        ("j-round-son-fc", "J Round & Son FC", 1875, "Tudor Street", "From Tudor Works, Tudor Street, now Tudor Sq", None, Some("S3")),
        ("dronfield-baptists-fc", "Dronfield Baptist FC", 1875, "Dronfield", "Church", None, Some("S18")),
        ("pye-bank-free-church-fc", "Pye Bank Free Church FC", 1875, "Pye Bank", "Pye Bank, Sheffield", None, Some("S3")),
        ("dronfield-independent-fc", "Dronfield Independent FC", 1875, "Dronfield", "From Independent Chapel Lea Road, Dronfield", None, Some("S18")),
        ("ebenezer-reform-fc", "Ebenezer Reform FC", 1875, "Unknown", "Ebenezer Chaple, Neepsend", None, Some("S6")),
        // These originally had region: None, now filled in based on location research
        ("clough-house-fc", "Clough House FC", 1875, "Clough area near Bramall Lane", "Likley from The Clough area near Bramall Lane", None, Some("S2")),  // Bramall Lane is S2
        ("trinity-fc", "Trinity FC", 1875, "Trinity Works", "From George Butler & Co, Trinity Works", None, Some("S3")),  // Trinity Works was in central Sheffield
        ("wostenholm-fc", "Wostenholm FC", 1875, "Washington Works", "club from Washington Works", None, Some("S3")),  // Washington Works on Washington Street = S3
        ("chester-brothers-fc", "Chester Brothers FC", 1875, "West End Cutlery Works, West Street", "From West End Cuterly Works West Street", None, Some("S1")),  // West Street = S1
        ("heeley-united-fc", "Heeley United FC", 1875, "Heeley", "From Heeley area", Some("Heeley"), Some("S2")),  // Heeley = S2
        ("minerva-fc", "Minerva FC", 1875, "John Street", "From their works on John Street", None, Some("S1")),  // John Street = central = S1
        ("kenyons-works-fc", "Kenyon's Works FC", 1875, "Hollins Crog", "From Kenyon Works, Hollins Crog", None, Some("S4")),  // Hollins Crog = Burngreave area = S4
        ("bee-hive-works-fc", "Bee Hive Works FC", 1875, "Bee Hive Works, Neepsend", "From Bee Hive Works Neepsend (not Milton St)", Some("Neepsend"), Some("S3")),
        ("cornish-place-fc", "Cornish Place FC", 1875, "Cornish Works, Neepsend", "From Cornish Works, Neepsend", Some("Neepsend"), Some("S3")),
        ("woodhouse-fc", "Woodhouse FC", 1875, "Woodhouse", "From Woodhouse", Some("Woodhouse"), Some("S13")),
    ];

    // Insert all clubs
    let mut stmt = conn.prepare(
        "INSERT INTO sheffield_clubs (id, name, founded_year, ground, origin, city, region)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    )?;

    let mut inserted = 0;
    for (id, name, year, ground, origin, city, region) in &clubs {
        stmt.execute(rusqlite::params![
            id,
            name,
            year,
            ground,
            origin,
            city,
            region
        ])?;
        inserted += 1;
    }

    eprintln!("Inserted {} clubs", inserted);

    // Show counts by year
    eprintln!("\n=== CLUBS BY YEAR ===\n");
    let mut stmt = conn.prepare(
        "SELECT founded_year, COUNT(*) as cnt FROM sheffield_clubs
         GROUP BY founded_year ORDER BY founded_year"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let year: i64 = row.get(0)?;
        let cnt: i64 = row.get(1)?;
        eprintln!("{}: {} clubs", year, cnt);
    }

    // Show counts by region
    eprintln!("\n=== CLUBS BY REGION ===\n");
    let mut stmt = conn.prepare(
        "SELECT region, COUNT(*) as cnt FROM sheffield_clubs
         WHERE region IS NOT NULL
         GROUP BY region ORDER BY cnt DESC"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let region: String = row.get(0)?;
        let cnt: i64 = row.get(1)?;
        eprintln!("{}: {} clubs", region, cnt);
    }

    // Check for any without region
    let no_region: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_clubs WHERE region IS NULL",
        [],
        |r| r.get(0)
    )?;
    eprintln!("\nClubs without region: {}", no_region);

    // Show sample clubs
    eprintln!("\n=== SAMPLE CLUBS (first 10) ===\n");
    let mut stmt = conn.prepare(
        "SELECT id, name, founded_year, ground, region FROM sheffield_clubs
         ORDER BY founded_year, name LIMIT 10"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let year: i64 = row.get(2)?;
        let ground: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let region: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        eprintln!("[{}] {} ({}) - {} @ {}", id, name, year, ground, region);
    }

    Ok(())
}
