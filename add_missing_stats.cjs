const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

try {
    console.log('Adding missing player stat columns...\n');

    const newColumns = [
        // Physical
        'acceleration',
        'natural_fitness',

        // Technical
        'technique',
        'corners',
        'free_kicks',
        'throw_ins',
        'vision',
        'left_foot',
        'right_foot',
        'one_on_ones',

        // Mental
        'bravery',
        'adaptability',
        'ambition',
        'loyalty',
        'pressure',
        'professionalism',
        'sportsmanship',
        'temperament',

        // Positioning
        'movement',

        // Hidden
        'injury_proneness',
        'important_matches'
    ];

    for (const column of newColumns) {
        try {
            db.exec(`ALTER TABLE sheffield_players ADD COLUMN ${column} INTEGER DEFAULT 10;`);
            console.log(`✓ Added ${column} column`);
        } catch (error) {
            if (error.message.includes('duplicate column name')) {
                console.log(`  ${column} already exists, skipping`);
            } else {
                throw error;
            }
        }
    }

    console.log('\n✓ Migration completed successfully!');

} catch (error) {
    console.error('Error:', error.message);
} finally {
    db.close();
}
