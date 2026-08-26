const fs = require('fs');

const clubs = [
  ['sheffield-fc', 'Sheffield FC', 1857, 'East Bank', 'played at East Bank'],
  ['hallam-fc', 'Hallam FC', 1860, 'Sandygate', 'play at Sandygate'],
  ['norfolk-fc', 'Norfolk FC', 1861, 'Norfolk Park', 'played at Norfolk Park'],
  ['cemetery-road-church-fc', 'Cemetery Road Church FC', 1861, 'Hunters Bar', 'Oldest church club, played at Hunters Bar'],
  ['york-fc', 'York FC', 1861, 'Endcliffe Cricket Ground', 'From York Hotel, Broomhill, played at Endcliffe Cricket Ground, N Creswick was President'],
  ['norton-fc', 'Norton FC', 1861, 'Oaks Park', 'played at Oaks Park, Norton'],
  ['pitsmoor-fc', 'Pitsmoor FC', 1861, 'Pitsmoor CC', 'played at Pitsmoor CC, now SUFC Academy'],
  ['fir-vale-fc', 'Fir Vale FC', 1862, 'Pitsmoor CC', 'played at Pitsmoor CC, now SUFC Academy'],
  ['heeley-christ-church-fc', 'Heeley Christ Church FC', 1862, 'Meersbrook Park', 'played at Meersbrook Park'],
  ['mackenzie-fc', 'Mackenzie FC', 1862, 'Myrtle Road', 'played at Myrtle Road, Heeley'],
  ['milton-fc', 'Milton FC', 1862, 'Cremorne Gardens', 'played at Cremorne Gardens, London Road'],
  ['howard-hill-steel-bank-fc', 'Howard Hill Steel Bank FC', 1862, 'Howard Hotel', 'Met in Howard Hotel, Howard Road'],
  ['ranmoor-fc', 'Ranmoor FC', 1862, 'Ranmoor', 'played in Ranmoor'],
  ['st-george-fc', 'St George FC', 1862, 'Broad Lane', 'From St George\'s Church, Broad Lane'],
  ['st-stephen-fc', 'St Stephen FC', 1862, 'Crookes', 'From St Stephen\'s Church Fawcett Road Netherthorpe. Played at Crookes'],
  ['united-norfolk-fc', 'United Norfolk FC', 1862, 'Unknown', 'Origins uncertain'],
  ['crabtree-fc', 'Crabtree FC', 1863, 'Fir Vale', 'Likely from Fir Vale area'],
  ['broomhall-fc', 'Broomhall FC', 1863, 'Ecclesall Road', 'played at Ecclesall Road'],
  ['tudor-fc', 'Tudor FC', 1863, 'Unknown', ''],
  ['wh-hutchinson-fc', 'W & H Hutchinson\'s FC', 1863, 'Unknown', ''],
  ['hemsworth-fc', 'Hemsworth FC', 1863, 'Unknown', ''],
  ['united-mechanics', 'United Mechanics', 1865, 'Norfolk Park', 'played at Norfolk Park'],
  ['garrick-fc', 'Garrick FC', 1866, 'East Bank', 'played at East Bank'],
  ['wellington-fc', 'Wellington FC', 1866, 'Hounsfield Park', 'played at Hounsfield Park near Bramall Lane'],
  ['loxley-fc', 'Loxley FC', 1866, 'The Rodney Inn', 'Met at The Rodney Inn, Loxley'],
  ['wednesday-fc', 'Wednesday FC', 1867, 'Highfields', 'played at Highfields, now Hillsborough'],
  ['exchange-fc', 'Exchange FC', 1867, 'Hallam\'s Farm', 'played at Hallam\'s Farm, now Hyde Park Flats'],
  ['dore-fc', 'Dore FC', 1867, 'The Devonshire Arms', 'Met at The Devonshire Arms, Dore'],
  ['tapton-fc', 'Tapton FC', 1867, 'Tapton Hall', 'Based at Tapton Hall'],
  ['dronfield-fc', 'Dronfield FC', 1868, 'Bagley\'s Field', 'played at Bagley\'s Field, Dronfield'],
  ['brincliffe-fc', 'Brincliffe FC', 1868, 'Cherry Tree Farm', 'played at Cherry Tree Farm, near Union pub'],
  ['hanover-united-fc', 'Hanover United FC', 1868, 'Crookes', 'played at Crookes'],
  ['stannington-fc', 'Stannington FC', 1868, 'Unknown', 'unknown ground location'],
  ['redhill-fc', 'Redhill FC', 1868, 'Winter Street', 'played at Winter Street, near Weston Park'],
  ['parkwood-springs-fc', 'Parkwood Springs FC', 1869, 'Parkwood Springs Recreation Ground', 'played at Parkwood Springs Recreation Ground'],
  ['oxford-fc', 'Oxford FC', 1869, 'Ecclesall Road', 'played at Ecclesall Road'],
  ['totley-fc', 'Totley FC', 1869, 'Cross Scythes Inn', 'played at field next to Cross Scythes Inn, Totley'],
  ['sheffield-norfolk-fc', 'Sheffield Norfolk FC', 1869, 'Unknown', ''],
  ['st-vincents', 'St Vincent\'s', 1869, 'Queens Ground', 'from Solly Street – played at Queens Ground'],
  ['st-james-church-fc', 'St James Church FC', 1869, 'Norton Lees Lane', 'played at Norton Lees Lane'],
  ['lockwood-brothers-fc', 'Lockwood Brothers FC', 1870, 'Hunters Bar', 'played at Hunters Bar, oldest works club'],
  ['talbot-fc', 'Talbot FC', 1870, 'Norfolk Road', 'played at Norfolk Road'],
  ['sheffield-united-gymnastic-club', 'Sheffield United Gymnastic Club', 1870, 'Unknown', ''],
  ['ecclesall-college-fc', 'Ecclesall College FC', 1870, 'Unknown', ''],
  ['sheffield-grammar-school-fc', 'Sheffield Grammar School FC', 1870, 'Unknown', ''],
  ['surrey-catholic-club', 'Surrey Catholic Club', 1870, 'The Farm', 'played at The Farm, now Sheffield College'],
  ['attercliffe-christ-church', 'Attercliffe (Christ Church)', 1870, 'The Old Forge Ground', 'played at The Old Forge Ground, Shirland Lane'],
  ['walkey-new-connexion-fc', 'Walkey New Connexion FC', 1870, 'Queens Ground', 'played at Queens Ground, Hillsborough'],
  ['crookes-fc', 'Crookes FC', 1870, 'Lydgate Lane', 'played at Lydgate Lane'],
  ['bankers-thursday', 'Bankers / Thursday', 1870, 'Hunters Bar', 'played at Hunters Bar'],
  ['alliance-fc', 'Alliance FC', 1870, 'Norfolk Park', 'played at Norfolk Park'],
  ['perseverance-fc', 'Perseverance FC', 1870, 'Norfolk Park', 'played at Norfolk Park'],
  ['gleadless-fc', 'Gleadless FC', 1870, 'Charnock Hall', 'played at Charnock Hall, Gleadless'],
  ['engineers-fc', 'Engineers FC', 1870, 'Endcliffe Crescent', 'Played at Endcliffe Crescent'],
  ['attercliffe-zion-fc', 'Attercliffe Zion FC', 1871, 'Attercliffe', 'from Zion Church, Attercliffe'],
  ['grimesthorpe-fc', 'Grimesthorpe FC', 1871, 'Victoria Hotel', 'Met at Victoria Hotel, Grimesthorpe'],
  ['exchange-brewery-fc', 'Exchange Brewery FC', 1871, 'Fox Street', 'played at Fox Street, Pye Bank'],
  ['all-saints-night-school-fc', 'All Saints Night School FC', 1871, 'Hall Carr Lane', 'Hall Carr Lane, now Carwood Road'],
  ['millhouses-fc', 'Millhouses FC', 1871, 'The Old Corn Mill', 'possibly played at The Old Corn Mill'],
  ['albion-fc', 'Albion FC', 1872, 'Ecclesall Road', 'played at Ecclesall Road'],
  ['pye-bank-fc', 'Pye Bank FC', 1872, 'Fox Street', 'played at Fox Street, Pye Bank'],
  ['brightside-fc', 'Brightside FC', 1872, 'Blackburn Meadows', 'played at Blackburn Meadows, Blackburn'],
  ['norfolk-works-fc', 'Norfolk Works FC', 1872, 'Newhall Athletic Ground', 'Played at Newhall Athletic Ground'],
  ['eldon-st-judes-fc', 'Eldon St Jude\'s FC', 1872, 'Brocco Bank', 'played at Brocco Bank'],
  ['garden-street-fc', 'Garden Street FC', 1872, 'Hollins Crog', 'From Garden Street, Hollins Crog'],
  ['sharrow-rangers-fc', 'Sharrow Rangers FC', 1873, 'Crescent Road', 'played at Crescent Road, Sharrow'],
  ['endcliffe-fc', 'Endcliffe FC', 1873, 'Ecclesall Road', 'played at Ecclesall Road'],
  ['owlerton-fc', 'Owlerton FC', 1873, 'Rawson\'s Meadow Ground', 'played at Rawson\'s Meadow Ground, Owlerton'],
  ['ecclesfield-fc', 'Ecclesfield FC', 1873, 'Fairham\'s Crog', 'played at Fairham\'s Crog'],
  ['philadelphia-fc', 'Philadelphia FC', 1873, 'Queens Ground', 'played at Queens Ground Hillsborough'],
  ['artillery-hallamshire-fc', 'Artillery & Hallamshire FC', 1873, 'Endcliffe Hall', 'Based at Endcliffe Hall (later just Artillery)'],
  ['intake-fc', 'Intake FC', 1873, 'Intake', 'From Intake'],
  ['cherrytree-fc', 'Cherrytree FC', 1873, 'Unknown', 'Possibly from Cherrytree Orphanage'],
  ['victoria-burngreave-fc', 'Victoria (Burngreave) FC', 1873, 'Hall Carr Lane', 'played at Hall Carr Lane, east end'],
  ['west-end-fc', 'West End FC', 1873, 'Hunters Bar', 'played at Hunters Bar, from West End Hotel'],
  ['burys-co-fc', 'Bury\'s & Co FC', 1873, 'Regents Works', 'From Regents Works, now Wicks'],
  ['beadshaws-baltic-fc', 'Beadshaw\'s (Baltic) FC', 1873, 'Baltic Works', 'From Baltic Works, Attercliffe'],
  ['oughtibridge-fc', 'Oughtibridge FC', 1873, 'Oughtibridge', 'From Oughtibridge'],
  ['weston-fc', 'Weston FC', 1873, 'Weston Hall', 'Possibly played at Weston Hall'],
  ['wardsend-steel-works-fc', 'Wardsend Steel Works FC', 1873, 'Herries Road', 'From Wardsend Steel Works, Herries Road'],
  ['owlerton-reform-fc', 'Owlerton Reform FC', 1873, 'Borough Road', 'From Wesleyan Reform Church on Borough Road, Owlerton'],
  ['roebuck-fc', 'Roebuck FC', 1873, 'East Bank', 'Roebuck pub, played at East Bank'],
  ['tennant-brothers-co-fc', 'Tennant Brothers & Co FC', 1873, 'Exchange Brewery', 'club from Exchange Brewery'],
  ['crosspool-rangers-fc', 'Crosspool Rangers FC', 1873, 'Crosspool', 'From Crosspool'],
  ['clifford-fc', 'Clifford FC', 1873, 'Psalter Lane', 'From Clifford House, Psalter Lane'],
  ['grange-fc', 'Grange FC', 1873, 'Intake Road', 'played at Intake Road'],
  ['pitsmoor-coal-company-fc', 'Pitsmoor Coal Company FC', 1873, 'Brightside', 'From Brightside'],
  ['boston-street-fc', 'Boston Street FC', 1873, 'Heeley', 'From Boston Street, played in Heeley'],
  ['birley-fc', 'Birley FC', 1873, 'Hollinsend', 'played at Hollinsend, Birley'],
  ['sherrington-fc', 'Sherrington FC', 1873, 'Norfolk Park', 'From Sherrington Road, Sharrow, played Norfolk Park'],
  ['105th-regiment', '105th Regiment', 1874, 'Unknown', 'Joined FA in 1874. Played in FA Cup 1875 to 1879. Nickname was \'Light Bobs\''],
  ['carnforth-fc', 'Carnforth FC', 1874, 'Sharrow Vale Road', 'played at Sharrow Vale Road'],
  ['hollinsend-fc', 'Hollinsend FC', 1874, 'Unknown', ''],
  ['providence-fc', 'Providence FC', 1874, 'Park Hill Lane', 'played at Park Hill Lane'],
  ['handsworth-fc', 'Handsworth FC', 1874, 'Unknown', 'Ground unknown'],
  ['woodseats-fc', 'Woodseats FC', 1874, 'Woodseats Hotel', 'Met at the Woodseats Hotel, now Viraaj'],
  ['oak-street-fc', 'Oak Street FC', 1874, 'Heeley', 'From street of same name, Heeley'],
  ['atlas-fc', 'Atlas FC', 1874, 'Atlas Works', 'formed from Atlas Works, East End'],
  ['ecclesall-fc', 'Ecclesall FC', 1874, 'Hunters Bar', 'played at Hunters Bar'],
  ['nether-fc', 'Nether FC', 1874, 'Eastborne', 'played at Eastborne'],
  ['broomfield-fc', 'Broomfield FC', 1874, 'Broomfield', 'From Broomfield area of Sheffield'],
  ['st-marks-fc', 'St Mark\'s FC', 1874, 'Broomfield Road', 'From St Mark\'s Broomfield Road'],
  ['tabernacle-fc', 'Tabernacle FC', 1874, 'Albert Terrace Road', 'From Tabernacle Church, Albert Terrace Road'],
  ['st-michaels-angels-fc', 'St Michaels Angels FC', 1874, 'Parkwood Springs', 'From St Michael\'s Church, Parkwood Springs'],
  ['kenwood-fc', 'Kenwood FC', 1874, 'Abbeydale Road', 'played at a ground off Abbeydale Road'],
  ['port-mahon-fc', 'Port Mahon FC', 1874, 'Port Mahon', 'From Port Mahon, now Ponderosa'],
  ['st-lukes-fc', 'St Luke\'s FC', 1874, 'Park', 'From St Luke\'s, Park (behind Midland Station)'],
  ['heeley-victoria-fc', 'Heeley Victoria FC', 1874, 'Victoria Hotel', 'From Victoria Hotel in Heeley'],
  ['shrewsbury-road-fc', 'Shrewsbury Road FC', 1874, 'Shrewsbury Road', 'Road of same name behind Midland station'],
  ['park-united-fc', 'Park United FC', 1874, 'Park', 'From Park area'],
  ['white-star-fc', 'White Star FC', 1874, 'Unknown', 'Location origins unknown'],
  ['good-intent-fc', 'Good Intent FC', 1874, 'Truro Ground', 'Likely played at Truro Ground on Matilda Street'],
  ['mount-tabor-fc', 'Mount Tabor FC', 1874, 'City Centre', 'Church in city centre, now demolish'],
  ['st-judes-fc', 'St Jude\'s FC', 1874, 'Cupola Street', 'From St Jude\'s church on Cupola Street'],
  ['wingfield-rowbotham-fc', 'Wingfield & Rowbotham FC', 1874, 'Tenter Street', 'From their works on Tenter Street'],
  ['young-broomhall-fc', 'Young Broomhall FC', 1874, 'Unknown', 'Ground location unknown'],
  ['regents-works-fc', 'Regents Works FC', 1874, 'Penistone Road', 'Regents Works, now Wicks on Penistone Road'],
  ['lo-good-templars-fc', 'L. O. Good Templars FC', 1874, 'Unknown', 'Origins unknown'],
  ['brookes-crookes-fc', 'Brookes & Crookes FC', 1874, 'Brook Lane', 'From Brook Steel Works, Brook Lane'],
  ['polar-star-fc', 'Polar Star FC', 1874, 'Norfolk Park', 'played at Norfolk Park'],
  ['norton-mount-view-fc', 'Norton Mount View FC', 1874, 'Norton Lees', 'From Mount View Methodists Church, Norton Lees'],
  ['mill-sands-fc', 'Mill Sands FC', 1874, 'Mill Sands Works', 'From Mill Sands Works, now Vulcan House'],
  ['fenton-brothers-fc', 'Fenton Brothers FC', 1874, 'East Street', 'From their works, East Street, Park'],
  ['sir-john-browns-fc', 'Sir John Brown\'s FC', 1874, 'Osgathorpe', 'played at Osgathorpe, near Earl Marshall'],
  ['st-silas-fc', 'St Silas FC', 1874, 'Broomhall', 'From St Silas Church, Broomhall'],
  ['ecclesall-church-fc', 'Ecclesall Church FC', 1874, 'Ecclesall', 'From Ecclesall All Saints'],
  ['firths-fc', 'Firth\'s FC', 1874, 'East End', 'From Thomas Firth & Son\'s, east end'],
  ['owlerton-united-fc', 'Owlerton United FC', 1874, 'Wadsley Bridge', 'played at Wadsley Bridge'],
  ['atkin-brothers-fc', 'Atkin Brothers FC', 1874, 'Matilda Street', 'From Truro Works, Matilda Street'],
  ['unitarian-fc', 'Unitarian FC', 1874, 'Norfolk Street', 'From Unitarian Church, Norfolk Street'],
  ['deep-pits-fc', 'Deep Pits FC', 1874, 'City Road', 'Near Manor Top, on City Road'],
  ['sheaf-fc', 'Sheaf FC', 1874, 'Sheaf Works', 'From Sheaf Works next to Victoria station'],
  ['montague-fc', 'Montague FC', 1874, 'Montague Street', 'From Montague Street Sharrow'],
  ['collegiate-fc', 'Collegiate FC', 1874, 'Unknown', '(exact date unknown)'],
  ['stag-home-fc', 'Stag Home FC', 1875, 'Kenwood', 'Origins uncertain, possibly played at Kenwood'],
  ['stanley-street-fc', 'Stanley Street FC', 1875, 'Norfolk Park', 'From Stanley Street, Wicker, played mainly at Norfolk Park'],
  ['langsett-road-fc', 'Langsett Road FC', 1875, 'Queens Ground', 'Likely played at Queens Ground'],
  ['manor-fc', 'Manor FC', 1875, 'Manor Lane', 'played at Manor Lane'],
  ['wheatman-smith-fc', 'Wheatman & Smith\'s FC', 1875, 'Russell Works', 'From Russell Works, near Kelham Island'],
  ['netherthorpe-fc', 'Netherthorpe FC', 1875, 'Unknown', 'Ground location uncertain'],
  ['st-peters-fc', 'St Peter\'s FC', 1875, 'Myrtle Road', 'From the now Cathedral, played at Myrtle Road'],
  ['st-philips-fc', 'St Philip\'s FC', 1875, 'Netherthorpe', 'From St Philip\'s church, Netherthorpe'],
  ['dronfield-united-fc', 'Dronfield United FC', 1875, 'Dronfield', 'From Dronfield'],
  ['hope-club', 'Hope Club', 1875, 'Weston Field', 'From Hope Works on Sussex Road, Played at Weston Field'],
  ['brown-bailey-dixon-fc', 'Brown Bailey & Dixon FC', 1875, 'Leeds Road', 'From their Attercliffe works, Leeds Road'],
  ['ward-payne-fc', 'Ward & Payne\'s FC', 1875, 'Hillsborough', 'From their Limbrick Works, Hillsborough'],
  ['crookes-united-fc', 'Crookes United FC', 1875, 'Lydgate Lane', 'Likely played at Lydgate Lane'],
  ['harold-fc', 'Harold FC', 1875, 'Walkley', 'Likely from Harold Street, Walkley'],
  ['hutton-son-fc', 'Hutton & Son FC', 1875, 'West Street', 'From Hutton\'s Works, West Street'],
  ['malinda-works-fc', 'Malinda Works FC', 1875, 'Malinda Street', 'From Malinda Works, Malinda Street'],
  ['parkwood-juniors-fc', 'Parkwood Juniors FC', 1875, 'Parkwood Springs', 'From Parkwood Springs'],
  ['aston-fc', 'Aston FC', 1875, 'Aston', 'From Aston'],
  ['washington-fc', 'Washington FC', 1875, 'Cobden View', 'From Wostenholm\'s Washington Works, Washington Street, poss played at Cobden View'],
  ['st-pauls-fc', 'St Paul\'s FC', 1875, 'Peace Gardens', 'From St Paul\'s church, now Peace Gardens'],
  ['south-view-fc', 'South View FC', 1875, 'Machon Bank', 'From South View Road in Sharrow, played at Machon Bank'],
  ['ecclesall-united-fc', 'Ecclesall United FC', 1875, 'Ecclesall', 'played at Ecclesall'],
  ['msl-loco-fc', 'MS&L Loco FC', 1875, 'Unknown', 'Ground location unknown'],
  ['oxford-wanderers-fc', 'Oxford Wanderers FC', 1875, 'Norfolk Park', 'Poss from Oxford Street, played Norfolk Park'],
  ['dronfield-free-church-fc', 'Dronfield Free Church FC', 1875, 'High Street', 'From Dronfield United Methodist Free Church, now Peel Centre, High Street, Dronfield'],
  ['atlantic-juniors-fc', 'Atlantic (Juniors) FC', 1875, 'Cobden View', 'played at Cobden View'],
  ['wincobank-fc', 'Wincobank FC', 1875, 'Wincobank', 'From Wincobank area'],
  ['alexandra-fc', 'Alexandra FC', 1875, 'Alexandra Road', 'From Alexandra Road, Heeley'],
  ['carbrook-united-fc', 'Carbrook United FC', 1875, 'Carbrook', 'played behind St Batholowmew\'s Church Carbrook'],
  ['r-sorby-son-fc', 'R Sorby & Son FC', 1875, 'Trafalgar Street', 'From Kangaroo Works, Trafalgar Street'],
  ['upperthorpe-fc', 'Upperthorpe FC', 1875, 'Cobden View', 'played at Cobden View'],
  ['howard-street-fc', 'Howard Street FC', 1875, 'Heeley', 'Likely played in Heeley, from Howard Street'],
  ['otley-sons-fc', 'Otley & Son\'s FC', 1875, 'Shalesmoor', 'From Meadow Works Shalesmoor'],
  ['hillsborough-fc', 'Hillsborough / Hillsborough School FC', 1875, 'Hillsborough', 'played at Hillsborough'],
  ['bellefield-fc', 'Bellefield FC', 1875, 'Bellefield Lane', 'From Bellefield Works, Bellefield Lane, Netherthorpe'],
  ['red-rose-fc', 'Red Rose FC', 1875, 'Unknown', 'Could be from Pitsmoor or Bramall Lane or was possibly 2 clubs'],
  ['weston-rovers-fc', 'Weston Rovers FC', 1875, 'Unknown', 'Uncertain origins'],
  ['j-round-son-fc', 'J Round & Son FC', 1875, 'Tudor Street', 'From Tudor Works, Tudor Street, now Tudor Sq'],
  ['dronfield-baptists-fc', 'Dronfield Baptists FC', 1875, 'Stubley Lane', 'From Dronfield Baptist Church, Stubley Lane'],
  ['pye-bank-free-church-fc', 'Pye Bank Free Church FC', 1875, 'Pye Bank', 'Pye Bank, Sheffield'],
  ['dronfield-independent-fc', 'Dronfield Independent FC', 1875, 'Lea Road', 'From Independent Chapel Lea Road, Dronfield'],
  ['ebernezer-reform-fc', 'Ebernezer Reform FC', 1875, 'Neepsend', 'Ebernezer Chaple, Neepsend'],
  ['kenyons-works-fc', 'Kenyon\'s Works FC', 1875, 'Unknown', ''],
  ['clough-house-fc', 'Clough House FC', 1875, 'The Clough', 'Likely from The Clough area near Bramall Lane'],
  ['trinity-fc', 'Trinity FC', 1875, 'Trinity Works', 'From George Butler & Co, Trinity Works'],
  ['wostenholm-fc', 'Wostenholm FC', 1875, 'Washington Works', 'club from Washington Works'],
  ['bee-hive-works-fc', 'Bee Hive Works FC', 1875, 'Neepsend', 'From Bee Hive Works Neepsend (not Milton St)'],
  ['chester-brothers-fc', 'Chester Brothers FC', 1875, 'West Street', 'From West End Cutlery Works West Street'],
  ['heeley-united-fc', 'Heeley United FC', 1875, 'Unknown', 'Unknown origins'],
  ['cornish-place-fc', 'Cornish Place FC', 1875, 'Neepsend', 'From Cornish Works, Neepsend'],
  ['minerva-fc', 'Minerva FC', 1875, 'John Street', 'From their works on John Street'],
  ['woodhouse-fc', 'Woodhouse FC', 1875, 'Woodhouse', 'From Woodhouse']
];

// Reserve team suffixes appropriate for 1850s-1870s era
const reserveSuffixes = ['Reserves', 'Juniors', 'Second Eleven', 'Casuals', 'B Side'];

function getReserveSuffix(index) {
  return reserveSuffixes[index % reserveSuffixes.length];
}

function getPostcode(name, ground, origin) {
  const text = `${name} ${ground} ${origin}`.toLowerCase();

  // S1 - City Centre
  if (text.includes('city centre') || text.includes('cathedral') || text.includes('peace gardens') ||
      text.includes('midland station') || text.includes('victoria station')) return 'S1';

  // S2 - Heeley, Manor, Norfolk Park, Park Hill, Highfield
  if (text.includes('heeley') || text.includes('manor') || text.includes('norfolk park') ||
      text.includes('park hill') || text.includes('highfield') || text.includes('meersbrook')) return 'S2';

  // S3 - Broomhall, Burngreave, Neepsend, Netherthorpe, Pitsmoor
  if (text.includes('broomhall') || text.includes('burngreave') || text.includes('neepsend') ||
      text.includes('netherthorpe') || text.includes('pitsmoor') || text.includes('fawcett road') ||
      text.includes('solly street')) return 'S3';

  // S4 - Brightside, Grimesthorpe, Osgathorpe, Pitsmoor
  if (text.includes('brightside') || text.includes('grimesthorpe') || text.includes('osgathorpe') ||
      text.includes('blackburn meadows')) return 'S4';

  // S5 - Fir Vale, Firth Park, Wincobank, Ecclesfield
  if (text.includes('fir vale') || text.includes('firth park') || text.includes('wincobank') ||
      text.includes('ecclesfield') || text.includes('parson cross')) return 'S5';

  // S6 - Hillsborough, Loxley, Stannington, Upperthorpe, Wadsley, Walkley, Owlerton
  if (text.includes('hillsborough') || text.includes('loxley') || text.includes('stannington') ||
      text.includes('upperthorpe') || text.includes('wadsley') || text.includes('walkley') ||
      text.includes('owlerton') || text.includes('queens ground') || text.includes('highfields')) return 'S6';

  // S7 - Millhouses, Nether Edge, Carter Knowle
  if (text.includes('millhouses') || text.includes('nether edge') || text.includes('carter knowle') ||
      text.includes('beauchief')) return 'S7';

  // S8 - Norton, Norton Lees, Woodseats, Meersbrook
  if (text.includes('norton') || text.includes('woodseats') || text.includes('batemoor') ||
      text.includes('greenhill')) return 'S8';

  // S9 - Attercliffe, Darnall, Handsworth, Tinsley
  if (text.includes('attercliffe') || text.includes('darnall') || text.includes('handsworth') ||
      text.includes('tinsley') || text.includes('shirland lane') || text.includes('leeds road')) return 'S9';

  // S10 - Broomhill, Crookes, Crosspool, Endcliffe, Ranmoor, Fulwood
  if (text.includes('broomhill') || text.includes('crookes') || text.includes('crosspool') ||
      text.includes('endcliffe') || text.includes('ranmoor') || text.includes('fulwood') ||
      text.includes('crookesmoor') || text.includes('hunters bar') || text.includes('lydgate lane')) return 'S10';

  // S11 - Ecclesall, Sharrow, Whirlow, Bents Green, Greystones
  if (text.includes('ecclesall') || text.includes('sharrow') || text.includes('whirlow') ||
      text.includes('bents green') || text.includes('greystones') || text.includes('ringinglow') ||
      text.includes('psalter lane') || text.includes('montague street') || text.includes('machon bank')) return 'S11';

  // S12 - Gleadless, Intake, Ridgeway, Hackenthorpe
  if (text.includes('gleadless') || text.includes('intake') || text.includes('ridgeway') ||
      text.includes('hackenthorpe') || text.includes('frecheville')) return 'S12';

  // S13 - Handsworth, Woodhouse, Woodthorpe
  if (text.includes('handsworth') || text.includes('woodhouse') || text.includes('woodthorpe') ||
      text.includes('orgreave') || text.includes('richmond')) return 'S13';

  // S17 - Dore, Totley
  if (text.includes('dore') || text.includes('totley') || text.includes('bradway')) return 'S17';

  // S18 - Dronfield
  if (text.includes('dronfield')) return 'S18';

  // S35 - Chapeltown, Ecclesfield, Grenoside, Oughtibridge, Worrall
  if (text.includes('chapeltown') || text.includes('grenoside') || text.includes('oughtibridge') ||
      text.includes('high green') || text.includes('worrall')) return 'S35';

  // S36 - Stocksbridge, Deepcar, Penistone
  if (text.includes('stocksbridge') || text.includes('deepcar') || text.includes('penistone') ||
      text.includes('oxspring') || text.includes('langsett')) return 'S36';

  // Specific location mappings
  if (text.includes('east bank')) return 'S2';
  if (text.includes('sandygate')) return 'S10';
  if (text.includes('bramall lane')) return 'S2';
  if (text.includes('weston')) return 'S10';
  if (text.includes('tapton')) return 'S10';
  if (text.includes('abbey')) return 'S7';
  if (text.includes('parkwood springs')) return 'S6';
  if (text.includes('carbrook')) return 'S9';
  if (text.includes('brincliffe')) return 'S11';
  if (text.includes('boston street')) return 'S2';
  if (text.includes('birley') || text.includes('hollinsend')) return 'S12';
  if (text.includes('herries road') || text.includes('wardsend')) return 'S6';
  if (text.includes('pye bank') || text.includes('fox street')) return 'S3';
  if (text.includes('kelham island') || text.includes('russell works')) return 'S3';
  if (text.includes('shalesmoor') || text.includes('meadow works')) return 'S3';
  if (text.includes('west street')) return 'S1';
  if (text.includes('washington works') || text.includes('wostenholm')) return 'S3';
  if (text.includes('bellefield')) return 'S3';
  if (text.includes('aston')) return 'S26';
  if (text.includes('hope works') || text.includes('hope club')) return 'S33';
  if (text.includes('cliff')) return 'S11';

  // Default to S1 (City Centre) if no match
  return 'S1';
}

function slugify(text) {
  return text.toLowerCase()
    .replace(/[^\w\s-]/g, '')
    .replace(/[\s_-]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

let sql = `-- Drop and recreate sheffield_clubs table with correct schema
DROP TABLE IF EXISTS sheffield_clubs;

CREATE TABLE sheffield_clubs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    short_name TEXT,
    founded_year INTEGER,
    ground_name TEXT,
    ground_capacity INTEGER,
    city TEXT,
    region TEXT,
    primary_color TEXT,
    secondary_color TEXT,
    origin TEXT,
    parent_club_id TEXT,
    is_reserve_team INTEGER DEFAULT 0,
    FOREIGN KEY (parent_club_id) REFERENCES sheffield_clubs(id)
);

-- Insert all ${clubs.length} Sheffield clubs
`;

// Insert main clubs
clubs.forEach(([id, name, year, ground, origin], idx) => {
  const shortName = name.split(' ').map(w => w[0]).join('').toUpperCase().substring(0, 3);
  const postcode = getPostcode(name, ground, origin);
  const region = postcode;
  const colors = ['#FF0000', '#0000FF', '#008000', '#800080', '#FFA500', '#8B4513', '#4B0082', '#228B22'];
  const primary = colors[idx % colors.length];
  const secondary = '#FFFFFF';

  sql += `INSERT INTO sheffield_clubs (id, name, short_name, founded_year, ground_name, ground_capacity, city, region, primary_color, secondary_color, origin, is_reserve_team) VALUES ('${id}', '${name.replace(/'/g, "''")}', '${shortName}', ${year}, '${ground.replace(/'/g, "''")}', 1000, 'Sheffield', '${region}', '${primary}', '${secondary}', '${origin.replace(/'/g, "''")}', 0);\n`;
});

sql += '\n-- Insert reserve teams for each club\n';

// Insert reserve teams
clubs.forEach(([parentId, parentName, year, ground, origin], idx) => {
  const suffix = getReserveSuffix(idx);
  const reserveName = `${parentName} ${suffix}`;
  const reserveId = `${parentId}-reserves`;
  const shortName = parentName.split(' ').map(w => w[0]).join('').toUpperCase().substring(0, 3) + 'R';
  const postcode = getPostcode(parentName, ground, origin);
  const region = postcode;
  const colors = ['#FF0000', '#0000FF', '#008000', '#800080', '#FFA500', '#8B4513', '#4B0082', '#228B22'];
  const primary = colors[idx % colors.length];
  const secondary = '#FFFFFF';

  sql += `INSERT INTO sheffield_clubs (id, name, short_name, founded_year, ground_name, ground_capacity, city, region, primary_color, secondary_color, origin, parent_club_id, is_reserve_team) VALUES ('${reserveId}', '${reserveName.replace(/'/g, "''")}', '${shortName}', ${year}, '${ground.replace(/'/g, "''")}', 500, 'Sheffield', '${region}', '${primary}', '${secondary}', 'Reserve team of ${parentName.replace(/'/g, "''")}', '${parentId}', 1);\n`;
});

sql += '\nSELECT COUNT(*) as total_clubs FROM sheffield_clubs;\n';
sql += 'SELECT COUNT(*) as reserve_teams FROM sheffield_clubs WHERE is_reserve_team = 1;\n';

fs.writeFileSync('./populate_sheffield_clubs_with_reserves.sql', sql);
console.log(`✓ Generated SQL with ${clubs.length} clubs and ${clubs.length} reserve teams (${clubs.length * 2} total)`);
