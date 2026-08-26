const fs = require('fs');

// Read the text file with notes
const textContent = fs.readFileSync('1857-1875.txt', 'utf-8');
const lines = textContent.split('\n').filter(l => l.trim());

// Build a map of normalized club name -> notes
const notesMap = {};
lines.forEach(line => {
    const parts = line.split('\t');
    if (parts.length >= 3) {
        const clubName = parts[0].trim();
        const notes = parts[2].trim();

        // Normalize ligatures to regular characters
        const normalized = clubName
            .replace(/ﬀ/g, 'ff')  // ff ligature
            .replace(/ﬁ/g, 'fi')  // fi ligature
            .replace(/ﬂ/g, 'fl')  // fl ligature
            .replace(/ﬃ/g, 'ffi') // ffi ligature
            .replace(/ﬄ/g, 'ffl') // ffl ligature
            .replace(/ﬆ/g, 'st'); // st ligature

        notesMap[normalized] = notes;
    }
});

console.log(`Loaded notes for ${Object.keys(notesMap).length} clubs (normalized)`);

// Read the Rust clubs.rs file
let rustContent = fs.readFileSync('src-tauri/src/sheffield_rules/clubs.rs', 'utf-8');

// Function to escape strings for use in regex
function escapeRegex(str) {
    return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

// Function to escape strings for Rust string literals
function escapeRustString(str) {
    return str.replace(/\\/g, '\\\\').replace(/"/g, '\\"');
}

// Manual mapping for clubs that still don't match
const manualMappings = {
    "Sheffield FC": "played at East Bank",
    "Hallam FC": "play at Sandygate",
    "Norfolk FC": "played at Norfolk Park",
    "Cemetery Road Church FC": "Oldest church club, played at Hunters Bar",
    "Norton FC": "played at Oaks Park, Norton",
    "Pitsmoor FC": "played at Pitsmoor CC, now SUFC Academy",
    "Fir Vale FC": "played at Pitsmoor CC, now SUFC Academy",
    "Heeley Christ Church FC": "played at Meersbrook Park",
    "Mackenzie FC": "played at Myrtle Road, Heeley",
    "Milton FC": "played at Cremorne Gardens, London Road",
    "Howard Hill Steel Bank FC": "Met in Howard Hotel, Howard Road",
    "Dronfield FC": "played at Bagley's Field, Dronfield",
    "Brincliffe FC": "played at Cherry Tree Farm, near Union pub",
    "Attercliffe (Christ Church)": "played at The Old Forge Ground, Shirland Lane",
    "St Vincent's": "from Solly Street – played at Queens Ground",
    "Attercliffe Zion FC": "from Zion Church, Attercliffe",
    "Eldon St Jude's FC": "played at Brocco Bank",
    "Endcliffe FC": "played at Ecclesall Road",
    "Ecclesfield FC": "played at Fairham's Crog",
    "Bury's & Co FC": "From Regents Works, now Wicks",
    "Beadshaw's (Baltic) FC": "From Baltic Works, Attercliffe",
    "Clifford FC": "From Clifford House, Psalter Lane",
    "Broomfield FC": "From Broomfield area of Sheffield",
    "St Mark's FC": "From St Mark's Broomfield Road",
    "St Luke's FC": "From St Luke's, Park (behind Midland Station),",
    "St Jude's FC": "From St Jude's church on Cupola Street",
    "Wingfield & Rowbotham FC": "From their works on Tenter Street",
    "Sir John Brown's FC": "played at Osgathorpe, near Earl Marshall",
    "Firth's FC": "From Thomas Firth & Son's, east end",
    "Wheatman & Smith's FC": "From Russell Works, near Kelham Island",
    "St Peter's FC": "From the now Cathedral, played at Myrtle Road",
    "St Philip's FC": "From St Philip's church, Netherthorpe",
    "Dronfield United FC": "From Dronfield",
    "Ward & Payne's FC": "From their Limbrick Works, Hillsborough",
    "Huffton & Son FC": "From Huffton's Works, West Street",
    "St Paul's FC": "From St Paul's church , now Peace Gardens",
    "Dronfield Free Church FC": "From Dronfield United Methodist Free Church, now Peel Centre, High Street, Dronfield",
    "Otley & Son's FC": "From Meadow Works Shalesmoor",
    "Bellefield FC": "From Bellefield Works, Bellefield Lane, Netherthorpe",
    "Dronfield Baptists FC": "From Dronfield Baptist Church, Stubley Lane",
    "Dronfield Independent FC": "From Independent Chapel Lea Road, Dronfield",
    "Ebenezer Reform FC": "Ebenezer Chaple, Neepsend",
    "Kenyon's Works FC": "From Kenyon Works, Hollins Crog",
};

// Combine with normalized map
const combinedMap = {...notesMap, ...manualMappings};

// Update each club's origin field
let updatesCount = 0;
for (const [clubName, notes] of Object.entries(combinedMap)) {
    if (!notes) continue; // Skip empty notes

    // Build regex to find this club's origin field
    const nameEscaped = escapeRegex(clubName);
    const pattern = new RegExp(
        `(name:\\s*"${nameEscaped}"[\\s\\S]*?origin:\\s*)"[^"]*"`,
        'g'
    );

    const notesEscaped = escapeRustString(notes);
    const replacement = `$1"${notesEscaped}"`;

    const before = rustContent;
    rustContent = rustContent.replace(pattern, replacement);

    if (before !== rustContent) {
        updatesCount++;
        console.log(`✓ Updated: ${clubName}`);
    }
}

// Write the updated content back
fs.writeFileSync('src-tauri/src/sheffield_rules/clubs.rs', rustContent);
console.log(`\nCompleted! Updated ${updatesCount} clubs total.`);
