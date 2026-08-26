const Database = require('better-sqlite3');
const db = new Database('D:/projects/Saturday at Three/Sheffield1867.db', { readonly: true });

console.log('\n=== Searching for events on Jul 8, 1867 and Jan 21, 1867 ===\n');

// Search for events with those dates
const dates = ['1867-07-08', '1867-01-08', '1867-01-21'];

dates.forEach(date => {
    console.log(`\n--- Looking for date: ${date} ---`);

    // Check news table
    try {
        const newsRows = db.prepare('SELECT * FROM news WHERE date = ?').all(date);
        if (newsRows.length > 0) {
            console.log(`Found ${newsRows.length} entries in NEWS table:`);
            newsRows.forEach(row => {
                console.log(JSON.stringify(row, null, 2));
            });
        } else {
            console.log('No entries in news table');
        }
    } catch (e) {
        console.log('news table error:', e.message);
    }

    // Check events table if it exists
    try {
        const eventRows = db.prepare('SELECT * FROM events WHERE date = ?').all(date);
        if (eventRows.length > 0) {
            console.log(`Found ${eventRows.length} entries in EVENTS table:`);
            eventRows.forEach(row => {
                console.log(JSON.stringify(row, null, 2));
            });
        }
    } catch (e) {
        // events table might not exist
    }
});

// Also search for any news items containing "dummy" in headline or content
console.log('\n\n--- Searching for news items with "dummy" in headline or content ---');
try {
    const dummyNews = db.prepare(`
        SELECT * FROM news
        WHERE headline LIKE '%dummy%'
           OR content LIKE '%dummy%'
        ORDER BY date
    `).all();

    if (dummyNews.length > 0) {
        console.log(`Found ${dummyNews.length} dummy news items:`);
        dummyNews.forEach(row => {
            console.log(JSON.stringify(row, null, 2));
        });
    } else {
        console.log('No dummy news items found');
    }
} catch (e) {
    console.log('Error searching for dummy items:', e.message);
}

db.close();
