// Shared utility functions for data import

// Name abbreviation expansion dictionary
const NAME_ABBREVIATIONS = {
    'Geo.': 'George', 'Geo': 'George',
    'Wm': 'William', 'Wm.': 'William',
    'Thos': 'Thomas', 'Thos.': 'Thomas',
    'Jas': 'James', 'Jas.': 'James',
    'Chas': 'Charles', 'Chas.': 'Charles',
    'Jno': 'John', 'Jno.': 'John',
    'Robt': 'Robert', 'Robt.': 'Robert',
    'Richd': 'Richard', 'Richd.': 'Richard',
    'Edwd': 'Edward', 'Edwd.': 'Edward',
    'Benj': 'Benjamin', 'Benj.': 'Benjamin',
    'Sam': 'Samuel', 'Sam.': 'Samuel',
    'Jos': 'Joseph', 'Jos.': 'Joseph',
    'Danl': 'Daniel', 'Danl.': 'Daniel',
    'Michl': 'Michael', 'Michl.': 'Michael',
    'Saml': 'Samuel', 'Saml.': 'Samuel',
    'Fredk': 'Frederick', 'Fredk.': 'Frederick',
    'Hy': 'Henry', 'Hy.': 'Henry',
    'Elizth': 'Elizabeth', 'Elizth.': 'Elizabeth',
    'Margt': 'Margaret', 'Margt.': 'Margaret',
    'Cath': 'Catherine', 'Cath.': 'Catherine',
};

// Profession abbreviation expansion dictionary
const PROFESSION_ABBREVIATIONS = {
    'agri lab': 'agricultural labourer',
    'agric lab': 'agricultural labourer',
    'lab': 'labourer',
    'servt': 'servant',
    'dom servt': 'domestic servant',
    'gen servt': 'general servant',
    'app': 'apprentice',
};

// Expand name abbreviations
function expandNameAbbreviations(name) {
    if (!name) return name;
    let expanded = name;
    for (const [abbrev, full] of Object.entries(NAME_ABBREVIATIONS)) {
        const regex = new RegExp(`\\b${abbrev.replace('.', '\\.')}\\b`, 'gi');
        expanded = expanded.replace(regex, full);
    }
    return expanded;
}

// Expand profession abbreviations
function expandProfessionAbbreviations(profession) {
    if (!profession) return profession;
    let expanded = profession.toLowerCase();
    for (const [abbrev, full] of Object.entries(PROFESSION_ABBREVIATIONS)) {
        const regex = new RegExp(`\\b${abbrev}\\b`, 'gi');
        expanded = expanded.replace(regex, full);
    }
    return capitalizeWords(expanded);
}

// Capitalize words: "cemetary road, barnsley" → "Cemetary Road, Barnsley"
function capitalizeWords(str) {
    if (!str) return str;
    return str.split(' ').map(word => {
        if (word.length === 0) return word;
        // Handle punctuation
        if (word.includes(',')) {
            return word.split(',').map(w => w.charAt(0).toUpperCase() + w.slice(1).toLowerCase()).join(',');
        }
        return word.charAt(0).toUpperCase() + word.slice(1).toLowerCase();
    }).join(' ');
}

// Normalize address: "19,SomersetRoad" → "19 Somerset Road"
// "shortstreet,yard,65" → "65 Yard Short Street"
// "~ cemetary road, barnsley" → "Cemetary Road, Barnsley"
function normalizeAddress(address) {
    if (!address) return { full: null, number: null, subArea: null, street: null };

    // Remove ~ and extra spaces
    let cleaned = address.replace(/~/g, '').trim();

    // Split by comma
    let parts = cleaned.split(',').map(p => p.trim()).filter(p => p);

    // Try to find house number
    let houseNumber = null;
    let remaining = [];

    for (const part of parts) {
        // Check if part starts with or is a number
        const numMatch = part.match(/^(\d+[a-z]?)\b/i);
        if (numMatch && !houseNumber) {
            houseNumber = numMatch[1];
            const rest = part.slice(numMatch[0].length).trim();
            if (rest) remaining.push(rest);
        } else {
            remaining.push(part);
        }
    }

    // Add spacing to concatenated words: "SomersetRoad" → "Somerset Road"
    remaining = remaining.map(part => {
        return part.replace(/([a-z])([A-Z])/g, '$1 $2');
    });

    // Capitalize each part
    remaining = remaining.map(capitalizeWords);

    let street = null;
    let subArea = null;

    if (remaining.length === 1) {
        street = remaining[0];
    } else if (remaining.length === 2) {
        subArea = remaining[0];
        street = remaining[1];
    } else if (remaining.length > 2) {
        // Last part is street, middle parts are sub-areas
        street = remaining[remaining.length - 1];
        subArea = remaining.slice(0, -1).join(' ');
    }

    // Build final address
    const fullParts = [];
    if (houseNumber) fullParts.push(houseNumber);
    if (subArea) fullParts.push(subArea);
    if (street) fullParts.push(street);

    return {
        full: fullParts.join(' ') || null,
        number: houseNumber,
        subArea: subArea,
        street: street
    };
}

// Split name into first, middle, surname
function splitName(fullName) {
    if (!fullName) return { first: null, middle: null, surname: null };

    const parts = fullName.trim().split(/\s+/);

    if (parts.length === 1) {
        return { first: parts[0], middle: null, surname: null };
    } else if (parts.length === 2) {
        return { first: parts[0], middle: null, surname: parts[1] };
    } else {
        return {
            first: parts[0],
            middle: parts.slice(1, -1).join(' '),
            surname: parts[parts.length - 1]
        };
    }
}

// Escape SQL strings
function sqlEscape(str) {
    if (str === null || str === undefined || str === '') return 'NULL';
    return `'${String(str).replace(/'/g, "''")}'`;
}

module.exports = {
    expandNameAbbreviations,
    expandProfessionAbbreviations,
    capitalizeWords,
    normalizeAddress,
    splitName,
    sqlEscape
};
