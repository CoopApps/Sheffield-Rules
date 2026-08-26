// Generate lists of all player stat fields for easy copying

const allStats = [
    // Core info
    'id', 'name', 'club_id', 'position', 'birth_year', 'age', 'nationality',

    // Physical
    'pace', 'acceleration', 'strength', 'stamina', 'balance', 'jumping', 'agility', 'natural_fitness',

    // Technical
    'passing', 'dribbling', 'first_touch', 'technique', 'heading', 'long_passing', 'crossing',
    'long_shots', 'tackling', 'handling', 'reflexes', 'corners', 'free_kicks', 'throw_ins',
    'vision', 'left_foot', 'right_foot', 'one_on_ones',

    // Mental
    'courage', 'bravery', 'concentration', 'decision_making', 'leadership', 'aggression',
    'anticipation', 'determination', 'flair', 'influence', 'adaptability', 'ambition',
    'loyalty', 'pressure', 'professionalism', 'sportsmanship', 'temperament',

    // Positioning
    'awareness', 'marking', 'positioning', 'work_rate', 'off_the_ball', 'movement', 'teamwork',

    // Specialization
    'finishing', 'penalties', 'set_pieces',

    // Hidden
    'consistency', 'dirtiness', 'versatility', 'injury_proneness', 'important_matches',

    // Ability & Reputation
    'current_ability', 'potential_ability', 'current_reputation'
];

console.log('Total fields:', allStats.length);
console.log('\n=== SQL SELECT ===');
console.log(allStats.join(', '));

console.log('\n=== Rust struct fields (excluding first 7) ===');
allStats.slice(7).forEach(field => {
    console.log(`    pub ${field}: i32,`);
});

console.log('\n=== TypeScript interface fields (excluding first 7) ===');
allStats.slice(7).forEach(field => {
    console.log(`  ${field}: number;`);
});
