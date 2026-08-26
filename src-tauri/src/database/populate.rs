use sqlx::SqlitePool;

/// Sheffield FC clubs data (1857-1875) - 187 historical clubs
pub struct SheffieldClub {
    pub id: &'static str,
    pub name: &'static str,
    pub founded_year: u32,
    pub ground_name: &'static str,
    pub origin: &'static str,
}

pub const SHEFFIELD_CLUBS: &[SheffieldClub] = &[
    SheffieldClub { id: "sheffield_fc_1857", name: "Sheffield FC", founded_year: 1857, ground_name: "East Bank", origin: "Original" },
    SheffieldClub { id: "hallam_fc_1860", name: "Hallam FC", founded_year: 1860, ground_name: "Sandygate", origin: "Original" },
    SheffieldClub { id: "norfolk_fc_1861", name: "Norfolk FC", founded_year: 1861, ground_name: "Norfolk Park", origin: "Hotel" },
    SheffieldClub { id: "cemetery_road_church_fc_1861", name: "Cemetery Road Church FC", founded_year: 1861, ground_name: "Hunters Bar", origin: "Church" },
    SheffieldClub { id: "york_fc_1861", name: "York FC", founded_year: 1861, ground_name: "Endcliffe Cricket Ground", origin: "Hotel" },
    SheffieldClub { id: "norton_fc_1861", name: "Norton FC", founded_year: 1861, ground_name: "Oaks Park", origin: "Local" },
    SheffieldClub { id: "pitsmoor_fc_1861", name: "Pitsmoor FC", founded_year: 1861, ground_name: "Pitsmoor CC", origin: "Local" },
    SheffieldClub { id: "fir_vale_fc_1862", name: "Fir Vale FC", founded_year: 1862, ground_name: "Pitsmoor CC", origin: "Works" },
    SheffieldClub { id: "heeley_christ_church_fc_1862", name: "Heeley Christ Church FC", founded_year: 1862, ground_name: "Meersbrook Park", origin: "Church" },
    SheffieldClub { id: "mackenzie_fc_1862", name: "Mackenzie FC", founded_year: 1862, ground_name: "Myrtle Road", origin: "Works" },
    SheffieldClub { id: "milton_fc_1862", name: "Milton FC", founded_year: 1862, ground_name: "Cremorne Gardens", origin: "Local" },
    SheffieldClub { id: "howard_hill_steel_bank_fc_1862", name: "Howard Hill Steel Bank FC", founded_year: 1862, ground_name: "Howard Road", origin: "Works" },
    SheffieldClub { id: "ranmoor_fc_1862", name: "Ranmoor FC", founded_year: 1862, ground_name: "Ranmoor", origin: "Local" },
    SheffieldClub { id: "st_george_fc_1862", name: "St George FC", founded_year: 1862, ground_name: "Broad Lane", origin: "Church" },
    SheffieldClub { id: "st_stephen_fc_1862", name: "St Stephen FC", founded_year: 1862, ground_name: "Crookes", origin: "Church" },
    SheffieldClub { id: "united_norfolk_fc_1862", name: "United Norfolk FC", founded_year: 1862, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "crabtree_fc_1863", name: "Crabtree FC", founded_year: 1863, ground_name: "Fir Vale", origin: "Local" },
    SheffieldClub { id: "broomhall_fc_1863", name: "Broomhall FC", founded_year: 1863, ground_name: "Ecclesall Road", origin: "Local" },
    SheffieldClub { id: "tudor_fc_1863", name: "Tudor FC", founded_year: 1863, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "w_h_hutchinson_fc_1863", name: "W & H Hutchinson's FC", founded_year: 1863, ground_name: "Unknown", origin: "Works" },
    SheffieldClub { id: "hemsworth_fc_1863", name: "Hemsworth FC", founded_year: 1863, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "united_mechanics_1865", name: "United Mechanics", founded_year: 1865, ground_name: "Norfolk Park", origin: "Works" },
    SheffieldClub { id: "garrick_fc_1866", name: "Garrick FC", founded_year: 1866, ground_name: "East Bank", origin: "Hotel" },
    SheffieldClub { id: "wellington_fc_1866", name: "Wellington FC", founded_year: 1866, ground_name: "Hounsfield Park", origin: "Hotel" },
    SheffieldClub { id: "loxley_fc_1866", name: "Loxley FC", founded_year: 1866, ground_name: "The Rodney Inn", origin: "Inn" },
    SheffieldClub { id: "wednesday_fc_1867", name: "Wednesday FC", founded_year: 1867, ground_name: "Highfields", origin: "Hotel" },
    SheffieldClub { id: "exchange_fc_1867", name: "Exchange FC", founded_year: 1867, ground_name: "Hyde Park Flats", origin: "Local" },
    SheffieldClub { id: "dore_fc_1867", name: "Dore FC", founded_year: 1867, ground_name: "The Devonshire Arms", origin: "Inn" },
    SheffieldClub { id: "tapton_fc_1867", name: "Tapton FC", founded_year: 1867, ground_name: "Tapton Hall", origin: "Local" },
    SheffieldClub { id: "dronfield_fc_1868", name: "Dronfield FC", founded_year: 1868, ground_name: "Bagley's Field", origin: "Local" },
    SheffieldClub { id: "brincliffe_fc_1868", name: "Brincliffe FC", founded_year: 1868, ground_name: "Cherry Tree Farm", origin: "Local" },
    SheffieldClub { id: "hanover_united_fc_1868", name: "Hanover United FC", founded_year: 1868, ground_name: "Crookes", origin: "Local" },
    SheffieldClub { id: "stannington_fc_1868", name: "Stannington FC", founded_year: 1868, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "redhill_fc_1868", name: "Redhill FC", founded_year: 1868, ground_name: "Winter Street", origin: "Local" },
    SheffieldClub { id: "parkwood_springs_fc_1869", name: "Parkwood Springs FC", founded_year: 1869, ground_name: "Parkwood Springs", origin: "Local" },
    SheffieldClub { id: "oxford_fc_1869", name: "Oxford FC", founded_year: 1869, ground_name: "Ecclesall Road", origin: "Local" },
    SheffieldClub { id: "totley_fc_1869", name: "Totley FC", founded_year: 1869, ground_name: "Cross Scythes Inn", origin: "Inn" },
    SheffieldClub { id: "sheffield_norfolk_fc_1869", name: "Sheffield Norfolk FC", founded_year: 1869, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "st_vincent_1869", name: "St Vincent's", founded_year: 1869, ground_name: "Queens Ground", origin: "Church" },
    SheffieldClub { id: "st_james_church_fc_1869", name: "St James Church FC", founded_year: 1869, ground_name: "Norton Lees Lane", origin: "Church" },
    SheffieldClub { id: "lockwood_brothers_fc_1870", name: "Lockwood Brothers FC", founded_year: 1870, ground_name: "Hunters Bar", origin: "Works" },
    SheffieldClub { id: "talbot_fc_1870", name: "Talbot FC", founded_year: 1870, ground_name: "Norfolk Road", origin: "Local" },
    SheffieldClub { id: "sheffield_united_gymnastic_club_1870", name: "Sheffield United Gymnastic Club", founded_year: 1870, ground_name: "Unknown", origin: "Club" },
    SheffieldClub { id: "ecclesall_college_fc_1870", name: "Ecclesall College FC", founded_year: 1870, ground_name: "Unknown", origin: "School" },
    SheffieldClub { id: "sheffield_grammar_school_fc_1870", name: "Sheffield Grammar School FC", founded_year: 1870, ground_name: "Unknown", origin: "School" },
    SheffieldClub { id: "surrey_catholic_club_1870", name: "Surrey Catholic Club", founded_year: 1870, ground_name: "The Farm", origin: "Church" },
    SheffieldClub { id: "aftercliffe_christ_church_1870", name: "Aftercliffe (Christ Church)", founded_year: 1870, ground_name: "The Old Forge Ground", origin: "Church" },
    SheffieldClub { id: "walkey_new_connexion_fc_1870", name: "Walkey New Connexion FC", founded_year: 1870, ground_name: "Queens Ground", origin: "Church" },
    SheffieldClub { id: "crookes_fc_1870", name: "Crookes FC", founded_year: 1870, ground_name: "Lydgate Lane", origin: "Local" },
    SheffieldClub { id: "bankers_thursday_1870", name: "Bankers / Thursday", founded_year: 1870, ground_name: "Hunters Bar", origin: "Club" },
    SheffieldClub { id: "alliance_fc_1870", name: "Alliance FC", founded_year: 1870, ground_name: "Norfolk Park", origin: "Local" },
    SheffieldClub { id: "perseverance_fc_1870", name: "Perseverance FC", founded_year: 1870, ground_name: "Norfolk Park", origin: "Local" },
    SheffieldClub { id: "gleadless_fc_1870", name: "Gleadless FC", founded_year: 1870, ground_name: "Charnock Hall", origin: "Local" },
    SheffieldClub { id: "engineers_fc_1870", name: "Engineers FC", founded_year: 1870, ground_name: "Endcliffe Crescent", origin: "Works" },
    SheffieldClub { id: "aftercliffe_zion_fc_1871", name: "Aftercliffe Zion FC", founded_year: 1871, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "grimesthorpe_fc_1871", name: "Grimesthorpe FC", founded_year: 1871, ground_name: "Victoria Hotel", origin: "Local" },
    SheffieldClub { id: "exchange_brewery_fc_1871", name: "Exchange Brewery FC", founded_year: 1871, ground_name: "Fox Street", origin: "Works" },
    SheffieldClub { id: "all_saints_night_school_fc_1871", name: "All Saints Night School FC", founded_year: 1871, ground_name: "Hall Carr Lane", origin: "School" },
    SheffieldClub { id: "millhouses_fc_1871", name: "Millhouses FC", founded_year: 1871, ground_name: "The Old Corn Mill", origin: "Local" },
    SheffieldClub { id: "albion_fc_1872", name: "Albion FC", founded_year: 1872, ground_name: "Ecclesall Road", origin: "Local" },
    SheffieldClub { id: "pye_bank_fc_1872", name: "Pye Bank FC", founded_year: 1872, ground_name: "Fox Street", origin: "Local" },
    SheffieldClub { id: "brightside_fc_1872", name: "Brightside FC", founded_year: 1872, ground_name: "Blackburn Meadows", origin: "Local" },
    SheffieldClub { id: "norfolk_works_fc_1872", name: "Norfolk Works FC", founded_year: 1872, ground_name: "Newhall Athletic Ground", origin: "Works" },
    SheffieldClub { id: "eldon_st_jude_fc_1872", name: "Eldon St Jude's FC", founded_year: 1872, ground_name: "Brocco Bank", origin: "Church" },
    SheffieldClub { id: "garden_street_fc_1872", name: "Garden Street FC", founded_year: 1872, ground_name: "Hollins Crog", origin: "Local" },
    SheffieldClub { id: "sharrow_rangers_fc_1873", name: "Sharrow Rangers FC", founded_year: 1873, ground_name: "Crescent Road", origin: "Local" },
    SheffieldClub { id: "endcliffe_fc_1873", name: "Endcliffe FC", founded_year: 1873, ground_name: "Ecclesall Road", origin: "Local" },
    SheffieldClub { id: "owlerton_fc_1873", name: "Owlerton FC", founded_year: 1873, ground_name: "Rawson's Meadow Ground", origin: "Local" },
    SheffieldClub { id: "ecclesfield_fc_1873", name: "Ecclesfield FC", founded_year: 1873, ground_name: "Fairham's Crog", origin: "Local" },
    SheffieldClub { id: "philadelphia_fc_1873", name: "Philadelphia FC", founded_year: 1873, ground_name: "Queens Ground", origin: "Local" },
    SheffieldClub { id: "artillery_hallamshire_fc_1873", name: "Artillery & Hallamshire FC", founded_year: 1873, ground_name: "Endcliffe Hall", origin: "Works" },
    SheffieldClub { id: "intake_fc_1873", name: "Intake FC", founded_year: 1873, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "cherrytree_fc_1873", name: "Cherrytree FC", founded_year: 1873, ground_name: "Unknown", origin: "Orphanage" },
    SheffieldClub { id: "victoria_burngreave_fc_1873", name: "Victoria (Burngreave) FC", founded_year: 1873, ground_name: "Hall Carr Lane", origin: "Local" },
    SheffieldClub { id: "west_end_fc_1873", name: "West End FC", founded_year: 1873, ground_name: "Hunters Bar", origin: "Hotel" },
    SheffieldClub { id: "bury_co_fc_1873", name: "Bury's & Co FC", founded_year: 1873, ground_name: "Regents Works", origin: "Works" },
    SheffieldClub { id: "beadshaw_baltic_fc_1873", name: "Beadshaw's (Baltic) FC", founded_year: 1873, ground_name: "Baltic Works", origin: "Works" },
    SheffieldClub { id: "oughtibridge_fc_1873", name: "Oughtibridge FC", founded_year: 1873, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "weston_fc_1873", name: "Weston FC", founded_year: 1873, ground_name: "Weston Hall", origin: "Local" },
    SheffieldClub { id: "wardsend_steel_works_fc_1873", name: "Wardsend Steel Works FC", founded_year: 1873, ground_name: "Herries Road", origin: "Works" },
    SheffieldClub { id: "owlerton_reform_fc_1873", name: "Owlerton Reform FC", founded_year: 1873, ground_name: "Borough Road", origin: "Church" },
    SheffieldClub { id: "roebuck_fc_1873", name: "Roebuck FC", founded_year: 1873, ground_name: "East Bank", origin: "Inn" },
    SheffieldClub { id: "tennant_brothers_co_fc_1873", name: "Tennant Brothers & Co FC", founded_year: 1873, ground_name: "Exchange Brewery", origin: "Works" },
    SheffieldClub { id: "crosspool_rangers_fc_1873", name: "Crosspool Rangers FC", founded_year: 1873, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "clifford_fc_1873", name: "Clifford FC", founded_year: 1873, ground_name: "Psalter Lane", origin: "Local" },
    SheffieldClub { id: "grange_fc_1873", name: "Grange FC", founded_year: 1873, ground_name: "Intake Road", origin: "Local" },
    SheffieldClub { id: "pitsmoor_coal_company_fc_1873", name: "Pitsmoor Coal Company FC", founded_year: 1873, ground_name: "Unknown", origin: "Works" },
    SheffieldClub { id: "boston_street_fc_1873", name: "Boston Street FC", founded_year: 1873, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "birley_fc_1873", name: "Birley FC", founded_year: 1873, ground_name: "Hollinsend", origin: "Local" },
    SheffieldClub { id: "sherrington_fc_1873", name: "Sherrington FC", founded_year: 1873, ground_name: "Norfolk Park", origin: "Local" },
    SheffieldClub { id: "105th_regiment_1874", name: "105th Regiment", founded_year: 1874, ground_name: "Unknown", origin: "Military" },
    SheffieldClub { id: "carnforth_fc_1874", name: "Carnforth FC", founded_year: 1874, ground_name: "Sharrow Vale Road", origin: "Local" },
    SheffieldClub { id: "hollinsend_fc_1874", name: "Hollinsend FC", founded_year: 1874, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "providence_fc_1874", name: "Providence FC", founded_year: 1874, ground_name: "Park Hill Lane", origin: "Local" },
    SheffieldClub { id: "handsworth_fc_1874", name: "Handsworth FC", founded_year: 1874, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "woodseats_fc_1874", name: "Woodseats FC", founded_year: 1874, ground_name: "The Woodseats Hotel", origin: "Hotel" },
    SheffieldClub { id: "oak_street_fc_1874", name: "Oak Street FC", founded_year: 1874, ground_name: "Heeley", origin: "Local" },
    SheffieldClub { id: "atlas_fc_1874", name: "Atlas FC", founded_year: 1874, ground_name: "Atlas Works", origin: "Works" },
    SheffieldClub { id: "ecclesall_fc_1874", name: "Ecclesall FC", founded_year: 1874, ground_name: "Hunters Bar", origin: "Local" },
    SheffieldClub { id: "nether_fc_1874", name: "Nether FC", founded_year: 1874, ground_name: "Eastborne", origin: "Local" },
    SheffieldClub { id: "broomfield_fc_1874", name: "Broomfield FC", founded_year: 1874, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "st_mark_fc_1874", name: "St Mark's FC", founded_year: 1874, ground_name: "Broomfield Road", origin: "Church" },
    SheffieldClub { id: "tabernacle_fc_1874", name: "Tabernacle FC", founded_year: 1874, ground_name: "Albert Terrace Road", origin: "Church" },
    SheffieldClub { id: "st_michael_angels_fc_1874", name: "St Michaels Angels FC", founded_year: 1874, ground_name: "Parkwood Springs", origin: "Church" },
    SheffieldClub { id: "kenwood_fc_1874", name: "Kenwood FC", founded_year: 1874, ground_name: "Abbeydale Road", origin: "Local" },
    SheffieldClub { id: "port_mahon_fc_1874", name: "Port Mahon FC", founded_year: 1874, ground_name: "Port Mahon", origin: "Local" },
    SheffieldClub { id: "st_luke_fc_1874", name: "St Luke's FC", founded_year: 1874, ground_name: "Park", origin: "Church" },
    SheffieldClub { id: "heeley_victoria_fc_1874", name: "Heeley Victoria FC", founded_year: 1874, ground_name: "Victoria Hotel", origin: "Hotel" },
    SheffieldClub { id: "shrewsbury_road_fc_1874", name: "Shrewsbury Road FC", founded_year: 1874, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "park_united_fc_1874", name: "Park United FC", founded_year: 1874, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "white_star_fc_1874", name: "White Star FC", founded_year: 1874, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "good_intent_fc_1874", name: "Good Intent FC", founded_year: 1874, ground_name: "Truro Ground", origin: "Local" },
    SheffieldClub { id: "mount_tabor_fc_1874", name: "Mount Tabor FC", founded_year: 1874, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "st_jude_fc_1874", name: "St Jude's FC", founded_year: 1874, ground_name: "Cupola Street", origin: "Church" },
    SheffieldClub { id: "wingfield_rowbotham_fc_1874", name: "Wingfield & Rowbotham FC", founded_year: 1874, ground_name: "Tenter Street", origin: "Works" },
    SheffieldClub { id: "young_broomhall_fc_1874", name: "Young Broomhall FC", founded_year: 1874, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "regents_works_fc_1874", name: "Regents Works FC", founded_year: 1874, ground_name: "Penistone Road", origin: "Works" },
    SheffieldClub { id: "l_o_good_templars_fc_1874", name: "L. O. Good Templars FC", founded_year: 1874, ground_name: "Unknown", origin: "Club" },
    SheffieldClub { id: "brookes_crookes_fc_1874", name: "Brookes & Crookes FC", founded_year: 1874, ground_name: "Brook Lane", origin: "Works" },
    SheffieldClub { id: "polar_star_fc_1874", name: "Polar Star FC", founded_year: 1874, ground_name: "Norfolk Park", origin: "Local" },
    SheffieldClub { id: "norton_mount_view_fc_1874", name: "Norton Mount View FC", founded_year: 1874, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "mill_sands_fc_1874", name: "Mill Sands FC", founded_year: 1874, ground_name: "Mill Sands Works", origin: "Works" },
    SheffieldClub { id: "fenton_brothers_fc_1874", name: "Fenton Brothers FC", founded_year: 1874, ground_name: "East Street", origin: "Works" },
    SheffieldClub { id: "sir_john_brown_fc_1874", name: "Sir John Brown's FC", founded_year: 1874, ground_name: "Osgathorpe", origin: "Works" },
    SheffieldClub { id: "st_silas_fc_1874", name: "St Silas FC", founded_year: 1874, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "ecclesall_church_fc_1874", name: "Ecclesall Church FC", founded_year: 1874, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "firth_fc_1874", name: "Firth's FC", founded_year: 1874, ground_name: "Unknown", origin: "Works" },
    SheffieldClub { id: "owlerton_united_fc_1874", name: "Owlerton United FC", founded_year: 1874, ground_name: "Wadsley Bridge", origin: "Local" },
    SheffieldClub { id: "atkin_brothers_fc_1874", name: "Atkin Brothers FC", founded_year: 1874, ground_name: "Matilda Street", origin: "Works" },
    SheffieldClub { id: "unitarian_fc_1874", name: "Unitarian FC", founded_year: 1874, ground_name: "Norfolk Street", origin: "Church" },
    SheffieldClub { id: "deep_pits_fc_1874", name: "Deep Pits FC", founded_year: 1874, ground_name: "City Road", origin: "Local" },
    SheffieldClub { id: "sheaf_fc_1874", name: "Sheaf FC", founded_year: 1874, ground_name: "Unknown", origin: "Works" },
    SheffieldClub { id: "montague_fc_1874", name: "Montague FC", founded_year: 1874, ground_name: "Sharrow", origin: "Local" },
    SheffieldClub { id: "collegiate_fc_pre1875", name: "Collegiate FC", founded_year: 1875, ground_name: "Unknown", origin: "School" },
    SheffieldClub { id: "stag_home_fc_1875", name: "Stag Home FC", founded_year: 1875, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "stanley_street_fc_1875", name: "Stanley Street FC", founded_year: 1875, ground_name: "Norfolk Park", origin: "Local" },
    SheffieldClub { id: "langsett_road_fc_1875", name: "Langsett Road FC", founded_year: 1875, ground_name: "Queens Ground", origin: "Local" },
    SheffieldClub { id: "manor_fc_1875", name: "Manor FC", founded_year: 1875, ground_name: "Manor Lane", origin: "Local" },
    SheffieldClub { id: "wheatman_smith_fc_1875", name: "Wheatman & Smith's FC", founded_year: 1875, ground_name: "Russell Works", origin: "Works" },
    SheffieldClub { id: "netherthorpe_fc_1875", name: "Netherthorpe FC", founded_year: 1875, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "st_peter_fc_1875", name: "St Peter's FC", founded_year: 1875, ground_name: "Myrtle Road", origin: "Church" },
    SheffieldClub { id: "st_philip_fc_1875", name: "St Philip's FC", founded_year: 1875, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "dronfield_united_fc_1875", name: "Dronfield United FC", founded_year: 1875, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "hope_club_1875", name: "Hope Club", founded_year: 1875, ground_name: "Weston Field", origin: "Works" },
    SheffieldClub { id: "brown_bailey_dixon_fc_1875", name: "Brown Bailey & Dixon FC", founded_year: 1875, ground_name: "Leeds Road", origin: "Works" },
    SheffieldClub { id: "ward_payne_fc_1875", name: "Ward & Payne's FC", founded_year: 1875, ground_name: "Limbrick Works", origin: "Works" },
    SheffieldClub { id: "crookes_united_fc_1875", name: "Crookes United FC", founded_year: 1875, ground_name: "Lydgate Lane", origin: "Local" },
    SheffieldClub { id: "harold_fc_1875", name: "Harold FC", founded_year: 1875, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "huffton_son_fc_1875", name: "Huffton & Son FC", founded_year: 1875, ground_name: "West Street", origin: "Works" },
    SheffieldClub { id: "malinda_works_fc_1875", name: "Malinda Works FC", founded_year: 1875, ground_name: "Malinda Street", origin: "Works" },
    SheffieldClub { id: "parkwood_juniors_fc_1875", name: "Parkwood Juniors FC", founded_year: 1875, ground_name: "Parkwood Springs", origin: "Local" },
    SheffieldClub { id: "aston_fc_1875", name: "Aston FC", founded_year: 1875, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "washington_fc_1875", name: "Washington FC", founded_year: 1875, ground_name: "Cobden View", origin: "Works" },
    SheffieldClub { id: "st_paul_fc_1875", name: "St Paul's FC", founded_year: 1875, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "south_view_fc_1875", name: "South View FC", founded_year: 1875, ground_name: "Machon Bank", origin: "Local" },
    SheffieldClub { id: "ecclesall_united_fc_1875", name: "Ecclesall United FC", founded_year: 1875, ground_name: "Ecclesall", origin: "Local" },
    SheffieldClub { id: "ms_l_loco_fc_1875", name: "MS&L Loco FC", founded_year: 1875, ground_name: "Unknown", origin: "Works" },
    SheffieldClub { id: "oxford_wanderers_fc_1875", name: "Oxford Wanderers FC", founded_year: 1875, ground_name: "Norfolk Park", origin: "Local" },
    SheffieldClub { id: "dronfield_free_church_fc_1875", name: "Dronfield Free Church FC", founded_year: 1875, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "atlantic_juniors_fc_1875", name: "Atlantic (Juniors) FC", founded_year: 1875, ground_name: "Cobden View", origin: "Local" },
    SheffieldClub { id: "wincobank_fc_1875", name: "Wincobank FC", founded_year: 1875, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "alexandra_fc_1875", name: "Alexandra FC", founded_year: 1875, ground_name: "Alexandra Road", origin: "Local" },
    SheffieldClub { id: "carbrook_united_fc_1875", name: "Carbrook United FC", founded_year: 1875, ground_name: "Carbrook", origin: "Church" },
    SheffieldClub { id: "r_sorby_son_fc_1875", name: "R Sorby & Son FC", founded_year: 1875, ground_name: "Trafalgar Street", origin: "Works" },
    SheffieldClub { id: "upperthorpe_fc_1875", name: "Upperthorpe FC", founded_year: 1875, ground_name: "Cobden View", origin: "Local" },
    SheffieldClub { id: "howard_street_fc_1875", name: "Howard Street FC", founded_year: 1875, ground_name: "Heeley", origin: "Local" },
    SheffieldClub { id: "otley_son_fc_1875", name: "Otley & Son's FC", founded_year: 1875, ground_name: "Shalesmoor", origin: "Works" },
    SheffieldClub { id: "hillsborough_fc_1875", name: "Hillsborough / Hillsborough School FC", founded_year: 1875, ground_name: "Hillsborough", origin: "School" },
    SheffieldClub { id: "bellefield_fc_1875", name: "Bellefield FC", founded_year: 1875, ground_name: "Netherthorpe", origin: "Works" },
    SheffieldClub { id: "red_rose_fc_1875", name: "Red Rose FC", founded_year: 1875, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "weston_rovers_fc_1875", name: "Weston Rovers FC", founded_year: 1875, ground_name: "Unknown", origin: "Local" },
    SheffieldClub { id: "j_round_son_fc_1875", name: "J Round & Son FC", founded_year: 1875, ground_name: "Tudor Street", origin: "Works" },
    SheffieldClub { id: "dronfield_baptists_fc_1875", name: "Dronfield Baptists FC", founded_year: 1875, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "pye_bank_free_church_fc_1875", name: "Pye Bank Free Church FC", founded_year: 1875, ground_name: "Pye Bank", origin: "Church" },
    SheffieldClub { id: "dronfield_independent_fc_1875", name: "Dronfield Independent FC", founded_year: 1875, ground_name: "Unknown", origin: "Church" },
    SheffieldClub { id: "ebenezer_reform_fc_1875", name: "Ebenezer Reform FC", founded_year: 1875, ground_name: "Unknown", origin: "Church" },
];

/// Populate clubs for a given year (ahistorical loads all, historical filters by founded_year)
pub async fn populate_clubs_for_year(
    pool: &SqlitePool,
    year: u32,
    is_ahistorical: bool,
) -> Result<(), sqlx::Error> {
    // Use the sheffield_rules clubs data which has the correct IDs
    use crate::sheffield_rules;

    let clubs_to_insert = sheffield_rules::get_sheffield_clubs(
        if is_ahistorical { None } else { Some(year) }
    );

    for club in clubs_to_insert {
        sqlx::query(
            "INSERT OR IGNORE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&club.id)
        .bind(&club.name)
        .bind(club.founded_year as i32)
        .bind(&club.ground)  // Note: sheffield_rules uses 'ground' field
        .bind(&club.origin)
        .bind(&club.city)
        .bind(&club.region)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Populate players from sheffield_footballers table (historical census data)
/// This reads real historical people from the Sheffield 1867 census who are eligible to play football
pub async fn populate_players_for_year(
    pool: &SqlitePool,
    year: u32,
) -> Result<(), sqlx::Error> {
    // Read footballers from the historical database
    // The sheffield_footballers table contains men aged 14-40 from the 1867 census
    // who aren't clergy, employers, or otherwise unsuitable for football

    let footballers = sqlx::query_as::<_, (i64, i64, Option<String>, Option<String>, Option<i64>, Option<String>, Option<String>)>(
        "SELECT id, person_id, first_name, surname, birth_year, profession, street_address
         FROM sheffield_footballers
         WHERE birth_year IS NOT NULL"
    )
    .fetch_all(pool)
    .await?;

    for (id, person_id, first_name, surname, birth_year, profession, street_address) in footballers {
        let first = first_name.clone().unwrap_or_default();
        let last = surname.clone().unwrap_or_default();
        let full_name = format!("{} {}", first, last).trim().to_string();

        if full_name.is_empty() {
            continue;
        }

        let birth_yr = birth_year.unwrap_or(year as i64 - 25) as i32;
        let age = year as i32 - birth_yr;

        // Skip if too young or too old for football
        if age < 14 || age > 45 {
            continue;
        }

        // Insert into sheffield_footballers with basic info from historical data
        // Stats will be generated when the player is assigned to a club
        sqlx::query(
            "INSERT OR IGNORE INTO sheffield_footballers
             (id, name, first_name, surname, birth_year, nationality, profession, street_address,
              census_age, census_gender, has_stats, is_real_player)
             VALUES (?, ?, ?, ?, ?, 'English', ?, ?, ?, 'Male', 0, 1)"
        )
        .bind(format!("footballer_{}", id))
        .bind(&full_name)
        .bind(&first_name)
        .bind(&surname)
        .bind(birth_yr)
        .bind(&profession)
        .bind(&street_address)
        .bind(age)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Create fixtures for a season (basic structure, fixture generation)
pub async fn create_fixtures_for_season(
    _pool: &SqlitePool,
    _season: u32,
    _clubs: Vec<String>,
) -> Result<(), sqlx::Error> {
    // TODO: Implement fixture generation (round-robin scheduling)
    // For now, this is a placeholder
    Ok(())
}

/// Initialize standings with zeros for all clubs
pub async fn initialize_standings(
    pool: &SqlitePool,
    season: u32,
    clubs: Vec<String>,
) -> Result<(), sqlx::Error> {
    let total_clubs = clubs.len();
    eprintln!("DEBUG: Initializing standings for {} clubs...", total_clubs);

    for (position, club_id) in clubs.iter().enumerate() {
        // Log progress every 50 clubs
        if position > 0 && position % 50 == 0 {
            eprintln!("DEBUG: Initialized {}/{} clubs ({:.0}%)",
                position, total_clubs, (position as f32 / total_clubs as f32) * 100.0);
        }

        sqlx::query(
            "INSERT INTO sheffield_standings (season, position, club_id, played, won, drawn, lost, goals_for, goals_against, goal_difference, rouges_for, rouges_against, points) VALUES (?, ?, ?, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)"
        )
        .bind(season as i32)
        .bind((position + 1) as i32)
        .bind(club_id)
        .execute(pool)
        .await?;
    }

    eprintln!("DEBUG: ✓ Completed initializing standings for all {} clubs", total_clubs);
    Ok(())
}
