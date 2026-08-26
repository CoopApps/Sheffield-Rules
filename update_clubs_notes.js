const fs = require('fs');

// Read the text file with notes
const textFile = fs.readFileSync('1857-1875.txt', 'utf-8');
const lines = textFile.split('\n').filter(line => line.trim());

// Parse club notes
const clubNotes = {};
lines.forEach(line => {
    const parts = line.split('\t');
    if (parts.length >= 3) {
        const name = parts[0].trim();
        const notes = parts[2].trim();
        clubNotes[name] = notes;
    }
});

console.log(`Parsed ${Object.keys(clubNotes).length} clubs with notes`);

// Read the Rust file
let rustFile = fs.readFileSync('src-tauri/src/sheffield_rules/clubs.rs', 'utf-8');

// Mapping of club names to their notes (need to match exactly)
const updates = [
    // Continuing from where we left off...
    { name: "Ranmoor FC", notes: "played in Ranmoor" },
    { name: "St George FC", notes: "From St George's Church, Broad Lane." },
    { name: "St Stephen FC", notes: "From St Stephen's Church Fawcett Road Netherthorpe. Played at Crookes." },
    { name: "United Norfolk FC", notes: "Origins uncertain" },
    { name: "Crabtree FC", notes: "Likely from Fir Vale area" },
    { name: "Broomhall FC", notes: "played at Ecclesall Road" },
    { name: "Tudor FC", notes: "" },
    { name: "W & H Hutchinson's FC", notes: "" },
    { name: "Hemsworth FC", notes: "" },
    { name: "United Mechanics", notes: "played at Norfolk Park" },
    { name: "Garrick FC", notes: "played at East Bank" },
    { name: "Wellington FC", notes: "played at Hounsfield Park near Bramall Lane" },
    { name: "Loxley FC", notes: "Met at The Rodney Inn, Loxley" },
    { name: "Wednesday FC", notes: "played at Highfields, now Hillsborough" },
    { name: "Exchange FC", notes: "played at Hallam's Farm, now Hyde Park Flats" },
    { name: "Dore FC", notes: "Met at The Devonshire Arms, Dore" },
    { name: "Tapton FC", notes: "Based at Tapton Hall" },
    { name: "Dronfield FC", notes: "played at Bagley's Field, Dronfield" },
    { name: "Brincliffe FC", notes: "played at Cherry Tree Farm, near Union pub" },
    { name: "Hanover United FC", notes: "played at Crookes" },
    { name: "Stannington FC", notes: "unknown ground location" },
    { name: "Redhill FC", notes: "played at Winter Street, near Weston Park" },
    { name: "Parkwood Springs FC", notes: "played at Parkwood Springs Recreation Ground" },
    { name: "Oxford FC", notes: "played at Ecclesall Road" },
    { name: "Totley FC", notes: "played at field next to Cross Scythes Inn, Totley" },
    { name: "Sheffield Norfolk FC", notes: "" },
    { name: "St Vincent's", notes: "from Solly Street – played at Queens Ground" },
    { name: "St James Church FC", notes: "played at Norton Lees Lane" },
    { name: "Lockwood Brothers FC", notes: "played at Hunters Bar, oldest works club" },
];

// Apply updates
updates.forEach(({name, notes}) => {
    if (!notes) return; // Skip empty notes

    // Find the club entry and update origin
    const escapedName = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const regex = new RegExp(
        `(name: "${escapedName}"[\\s\\S]*?origin: )"[^"]*"`,
        'g'
    );

    const escapedNotes = notes.replace(/\\/g, '\\\\').replace(/"/g, '\\"');
    rustFile = rustFile.replace(regex, `$1"${escapedNotes}"`);
});

// Write back
fs.writeFileSync('src-tauri/src/sheffield_rules/clubs.rs', rustFile);
console.log('Updated clubs.rs file');
