const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Analyzing Father-Daughter Matching Potential');
console.log('='.repeat(70) + '\n');

function getSurname(fullName) {
    if (!fullName) return '';
    const parts = fullName.trim().split(/\s+/);
    return parts[parts.length - 1];
}

const daughters = db.prepare('SELECT * FROM unmatched_genealogy WHERE relation = ?').all('Daughter');
console.log('Total daughters in unmatched_genealogy:', daughters.length.toLocaleString());
console.log('');

const sheffieldMen = db.prepare('SELECT * FROM sheffield_people WHERE census_gender = ?').all('Male');
console.log('Total men in sheffield_people:', sheffieldMen.length.toLocaleString());
console.log('');

let uniqueMatches = 0;
let multipleMatches = 0;
let noMatches = 0;
const examples = [];
const multipleExamples = [];

daughters.forEach((daughter, index) => {
    if (index % 500 === 0) {
        console.log(`Progress: ${index} / ${daughters.length}`);
    }

    const daughterSurname = getSurname(daughter.name);
    const potentialFathers = sheffieldMen.filter(man => {
        const manSurname = getSurname(man.name);
        if (manSurname !== daughterSurname) return false;
        if (!man.census_household_members) return false;

        const members = man.census_household_members.split(',').map(n => n.trim());
        return members.includes(daughter.name);
    });

    if (potentialFathers.length === 1) {
        uniqueMatches++;
        if (examples.length < 10) {
            examples.push({
                daughter: daughter.name,
                daughterYear: daughter.year,
                daughterAge: daughter.age,
                father: potentialFathers[0].name,
                fatherYear: potentialFathers[0].year,
                fatherAge: potentialFathers[0].age,
                household: potentialFathers[0].census_household_members
            });
        }
    } else if (potentialFathers.length > 1) {
        multipleMatches++;
        if (multipleExamples.length < 5) {
            multipleExamples.push({
                daughter: daughter.name,
                daughterYear: daughter.year,
                fathers: potentialFathers.map(f => ({
                    name: f.name,
                    year: f.year,
                    age: f.age,
                    household: f.census_household_members
                }))
            });
        }
    } else {
        noMatches++;
    }
});

console.log('\n' + '='.repeat(70));
console.log('RESULTS');
console.log('='.repeat(70));
console.log(`Unique matches (1 father): ${uniqueMatches.toLocaleString()}`);
console.log(`Multiple matches (2+ fathers): ${multipleMatches.toLocaleString()}`);
console.log(`No matches: ${noMatches.toLocaleString()}`);

console.log('\n' + '='.repeat(70));
console.log('EXAMPLE UNIQUE MATCHES');
console.log('='.repeat(70));
examples.forEach((ex, i) => {
    console.log(`\n${i+1}. Daughter: ${ex.daughter} (born ${ex.daughterYear}, age ${ex.daughterAge})`);
    console.log(`   Father: ${ex.father} (born ${ex.fatherYear}, age ${ex.fatherAge})`);
    console.log(`   Household: ${ex.household.substring(0, 100)}...`);
});

console.log('\n' + '='.repeat(70));
console.log('EXAMPLE MULTIPLE MATCHES (Ambiguous)');
console.log('='.repeat(70));
multipleExamples.forEach((ex, i) => {
    console.log(`\n${i+1}. Daughter: ${ex.daughter} (${ex.daughterYear})`);
    console.log(`   Potential fathers (${ex.fathers.length}):`);
    ex.fathers.forEach((f, j) => {
        console.log(`     ${j+1}. ${f.name} (${f.year}, age ${f.age})`);
    });
});

db.close();
console.log('\n✓ Analysis complete!');
