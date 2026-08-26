// Generate the stats object for handleSavePlayer

const fields = [
    'position',
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

console.log('stats: {');
fields.forEach(field => {
    console.log(`          ${field}: editingPlayer.${field},`);
});
console.log('        },');
