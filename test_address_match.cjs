function normalizeAddress(addr) {
    if (!addr) return '';
    return addr.toLowerCase()
        .replace(/~/g, '')
        // Add space before common street suffixes if not already there
        .replace(/([a-z])(road|street|lane|avenue|drive|terrace|place|square|row|grove|crescent|court|hill|way)/gi, '$1 $2')
        .replace(/\s+/g, ' ')
        .replace(/,/g, ' ')
        .replace(/\./g, '')
        // Now replace the full words with abbreviations
        .replace(/\broad\b/g, 'rd')
        .replace(/\bstreet\b/g, 'st')
        .replace(/\bavenue\b/g, 'ave')
        .replace(/\blane\b/g, 'ln')
        .replace(/\bdrive\b/g, 'dr')
        .replace(/\bterrace\b/g, 'ter')
        .replace(/\bplace\b/g, 'pl')
        .replace(/\bsquare\b/g, 'sq')
        .trim();
}

const addr1 = '156 Infirmary Road';
const addr2 = '156,InfirmaryRoad';

const norm1 = normalizeAddress(addr1);
const norm2 = normalizeAddress(addr2);

console.log('Business address: "' + addr1 + '"');
console.log('Normalized: "' + norm1 + '"');
console.log('');
console.log('Person address: "' + addr2 + '"');
console.log('Normalized: "' + norm2 + '"');
console.log('');
console.log('norm1.includes(norm2):', norm1.includes(norm2));
console.log('norm2.includes(norm1):', norm2.includes(norm1));
console.log('Match?', norm1.includes(norm2) || norm2.includes(norm1));
