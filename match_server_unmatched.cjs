const http = require('http');
const Database = require('better-sqlite3');
const crypto = require('crypto');

const db = new Database('Sheffield1867.db');
const PORT = 3001;

// Enable CORS
const setCorsHeaders = (res) => {
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
};

const server = http.createServer((req, res) => {
    setCorsHeaders(res);

    // Handle preflight
    if (req.method === 'OPTIONS') {
        res.writeHead(200);
        res.end();
        return;
    }

    // Get available years
    if (req.url === '/years' && req.method === 'GET') {
        const years = db.prepare(`
            SELECT DISTINCT year FROM unmatched_sheffield
            ORDER BY year
        `).all().map(r => r.year);

        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(years));
        return;
    }

    // Get records
    if (req.url.startsWith('/records') && req.method === 'GET') {
        const url = new URL(req.url, `http://localhost:${PORT}`);
        const year = url.searchParams.get('year');
        const limit = parseInt(url.searchParams.get('limit')) || 100;
        const sheffieldSearch = url.searchParams.get('sheffield_search') || '';
        const genealogySearch = url.searchParams.get('genealogy_search') || '';

        // Build Sheffield query
        let sheffieldQuery = 'SELECT * FROM unmatched_sheffield WHERE 1=1';
        const sheffieldParams = [];

        if (year) {
            sheffieldQuery += ' AND year = ?';
            sheffieldParams.push(year);
        }
        if (sheffieldSearch) {
            sheffieldQuery += ' AND name LIKE ?';
            sheffieldParams.push(`%${sheffieldSearch}%`);
        }
        sheffieldQuery += ' ORDER BY name LIMIT ?';
        sheffieldParams.push(limit);

        // Build Genealogy query
        let genealogyQuery = 'SELECT * FROM unmatched_genealogy WHERE 1=1';
        const genealogyParams = [];

        if (year) {
            genealogyQuery += ' AND year = ?';
            genealogyParams.push(year);
        }
        if (genealogySearch) {
            genealogyQuery += ' AND name LIKE ?';
            genealogyParams.push(`%${genealogySearch}%`);
        }
        genealogyQuery += ' ORDER BY name LIMIT ?';
        genealogyParams.push(limit);

        // Get records
        const sheffield = db.prepare(sheffieldQuery).all(...sheffieldParams);
        const genealogy = db.prepare(genealogyQuery).all(...genealogyParams);

        // Get total counts
        let totalSheffieldQuery = 'SELECT COUNT(*) as count FROM unmatched_sheffield WHERE 1=1';
        const totalSheffieldParams = [];
        if (year) {
            totalSheffieldQuery += ' AND year = ?';
            totalSheffieldParams.push(year);
        }
        if (sheffieldSearch) {
            totalSheffieldQuery += ' AND name LIKE ?';
            totalSheffieldParams.push(`%${sheffieldSearch}%`);
        }

        let totalGenealogyQuery = 'SELECT COUNT(*) as count FROM unmatched_genealogy WHERE 1=1';
        const totalGenealogyParams = [];
        if (year) {
            totalGenealogyQuery += ' AND year = ?';
            totalGenealogyParams.push(year);
        }
        if (genealogySearch) {
            totalGenealogyQuery += ' AND name LIKE ?';
            totalGenealogyParams.push(`%${genealogySearch}%`);
        }

        const totalSheffield = db.prepare(totalSheffieldQuery).get(...totalSheffieldParams).count;
        const totalGenealogy = db.prepare(totalGenealogyQuery).get(...totalGenealogyParams).count;

        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({
            sheffield,
            genealogy,
            stats: {
                total_sheffield: totalSheffield,
                total_genealogy: totalGenealogy
            }
        }));
        return;
    }

    // Save match
    if (req.url === '/match' && req.method === 'POST') {
        let body = '';
        req.on('data', chunk => {
            body += chunk.toString();
        });

        req.on('end', () => {
            try {
                const { sheffieldId, genealogyId } = JSON.parse(body);

                // Get both records
                const sheffield = db.prepare('SELECT * FROM unmatched_sheffield WHERE id = ?').get(sheffieldId);
                const genealogy = db.prepare('SELECT * FROM unmatched_genealogy WHERE id = ?').get(genealogyId);

                if (!sheffield || !genealogy) {
                    res.writeHead(404, { 'Content-Type': 'application/json' });
                    res.end(JSON.stringify({ error: 'Records not found' }));
                    return;
                }

                // Parse names
                function parseName(name) {
                    const parts = name.trim().split(/\s+/);
                    if (parts.length === 1) {
                        return { first_name: parts[0], middle_name: null, surname: null };
                    } else if (parts.length === 2) {
                        return { first_name: parts[0], middle_name: null, surname: parts[1] };
                    } else {
                        return {
                            first_name: parts[0],
                            middle_name: parts.slice(1, -1).join(' '),
                            surname: parts[parts.length - 1]
                        };
                    }
                }

                const nameParts = parseName(sheffield.name);

                // Create combined record
                const personId = crypto.randomUUID();
                const source = `${sheffield.year}_matched_manual`;

                db.prepare(`
                    INSERT INTO sheffield_people (
                        id, name, first_name, middle_name, surname, source,
                        birth_year, gender,
                        census_age, census_birth_place, census_county,
                        census_relation, census_gender, census_ed,
                        census_household_members, census_piece, census_folio, census_page,
                        where_born, civil_parish, ecclesiastical_parish,
                        registration_district, street_address, profession
                    ) VALUES (
                        ?, ?, ?, ?, ?, ?,
                        ?, ?,
                        ?, ?, ?,
                        ?, ?, ?,
                        ?, ?, ?, ?,
                        ?, ?, ?,
                        ?, ?, ?
                    )
                `).run(
                    personId,
                    sheffield.name,
                    nameParts.first_name,
                    nameParts.middle_name,
                    nameParts.surname,
                    source,
                    sheffield.birth_year || genealogy.birth_year,
                    sheffield.gender,
                    sheffield.age,
                    sheffield.birth_place,
                    sheffield.county,
                    sheffield.relation,
                    sheffield.gender,
                    null, // census_ed
                    sheffield.household_members,
                    sheffield.piece,
                    sheffield.folio,
                    sheffield.page,
                    sheffield.birth_place,
                    sheffield.civil_parish,
                    sheffield.ecclesiastical_parish,
                    sheffield.registration_district,
                    genealogy.address,
                    genealogy.profession
                );

                // Remove from unmatched tables
                db.prepare('DELETE FROM unmatched_sheffield WHERE id = ?').run(sheffieldId);
                db.prepare('DELETE FROM unmatched_genealogy WHERE id = ?').run(genealogyId);

                console.log(`✓ Matched: ${sheffield.name} (${sheffield.year})`);

                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ success: true, id: personId }));
            } catch (error) {
                console.error('Error saving match:', error);
                res.writeHead(500, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: error.message }));
            }
        });
        return;
    }

    // 404
    res.writeHead(404, { 'Content-Type': 'text/plain' });
    res.end('Not found');
});

server.listen(PORT, () => {
    console.log(`Match server running on http://localhost:${PORT}`);
    console.log('Open match_unmatched.html in your browser to start matching');
});
