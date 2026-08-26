const Database = require('better-sqlite3');
const fs = require('fs');

const dbPaths = [
    'D:/projects/Saturday at Three/Sheffield1867.db',
    'D:/projects/Saturday at Three/Sheffield1867_temp.db',
    'D:/projects/Saturday at Three/saturday_at_three.db'
];

const targetDates = ['1867-07-08', '1867-01-08', '1867-01-21'];

dbPaths.forEach(dbPath => {
    if (!fs.existsSync(dbPath)) {
        console.log(`\n❌ Database not found: ${dbPath}`);
        return;
    }

    console.log(`\n${'='.repeat(80)}`);
    console.log(`📁 Database: ${dbPath}`);
    console.log('='.repeat(80));

    const db = new Database(dbPath, { readonly: true });

    // Check for sheffield_news_items table
    const tableCheck = db.prepare(`
        SELECT name FROM sqlite_master
        WHERE type='table' AND name = 'sheffield_news_items'
    `).get();

    if (!tableCheck) {
        console.log('⚠️  No sheffield_news_items table found');

        // Check for regular 'news' table
        const newsTableCheck = db.prepare(`
            SELECT name FROM sqlite_master
            WHERE type='table' AND name = 'news'
        `).get();

        if (newsTableCheck) {
            console.log('✅ Found "news" table instead');
            console.log('\n--- Checking for target dates in "news" table ---');
            targetDates.forEach(date => {
                const rows = db.prepare('SELECT * FROM news WHERE date = ? OR publish_date = ?').all(date, date);
                if (rows.length > 0) {
                    console.log(`\n📅 Date: ${date} (${rows.length} items found):`);
                    rows.forEach(row => {
                        console.log(JSON.stringify(row, null, 2));
                    });
                }
            });

            // Check for "dummy" content
            console.log('\n--- Searching for "dummy" content in "news" table ---');
            const dummyRows = db.prepare(`
                SELECT * FROM news
                WHERE headline LIKE '%dummy%'
                   OR content LIKE '%dummy%'
                   OR body_text LIKE '%dummy%'
            `).all();

            if (dummyRows.length > 0) {
                console.log(`🎯 Found ${dummyRows.length} dummy items:`);
                dummyRows.forEach(row => {
                    console.log(JSON.stringify(row, null, 2));
                });
            }
        } else {
            console.log('⚠️  No "news" table found either');
        }

        db.close();
        return;
    }

    console.log('✅ Found sheffield_news_items table');

    // Get total count
    const count = db.prepare('SELECT COUNT(*) as count FROM sheffield_news_items').get();
    console.log(`📊 Total news items: ${count.count}`);

    if (count.count === 0) {
        console.log('   (Table is empty)');
        db.close();
        return;
    }

    // Check for target dates
    console.log('\n--- Checking for target dates ---');
    targetDates.forEach(date => {
        const rows = db.prepare('SELECT * FROM sheffield_news_items WHERE publish_date = ?').all(date);
        if (rows.length > 0) {
            console.log(`\n📅 Date: ${date} (${rows.length} items found):`);
            rows.forEach(row => {
                console.log(JSON.stringify(row, null, 2));
            });
        }
    });

    // Check for "dummy" content
    console.log('\n--- Searching for "dummy" content ---');
    const dummyRows = db.prepare(`
        SELECT * FROM sheffield_news_items
        WHERE headline LIKE '%dummy%'
           OR body_text LIKE '%dummy%'
    `).all();

    if (dummyRows.length > 0) {
        console.log(`🎯 Found ${dummyRows.length} dummy items:`);
        dummyRows.forEach(row => {
            console.log(JSON.stringify(row, null, 2));
        });
    } else {
        console.log('No dummy content found');
    }

    // Show all items
    console.log('\n--- All news items (first 10) ---');
    const allRows = db.prepare('SELECT * FROM sheffield_news_items LIMIT 10').all();
    if (allRows.length > 0) {
        allRows.forEach(row => {
            console.log(`📰 ${row.publish_date} - ${row.headline}`);
        });
    }

    db.close();
});

console.log(`\n${'='.repeat(80)}\n`);
