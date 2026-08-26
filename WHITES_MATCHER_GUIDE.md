# Whites Directory Matcher - User Guide

## Overview

The Whites Directory Matcher is a powerful GUI tool for matching business directory entries from Whites Directory to people in the Sheffield database. It uses sophisticated fuzzy matching algorithms and supports both automatic suggestions and manual drag-and-drop matching.

## Features

### 1. **Automatic Match Suggestions**
- Analyzes name, address, profession, and parish data
- Calculates confidence scores (0-100%) for potential matches
- Provides detailed match reasoning
- Configurable confidence threshold

### 2. **Manual Drag-and-Drop Matching**
- Drag any person from the right panel to a business entry on the left
- Visual feedback during drag operations
- Confirmation dialog before accepting matches

### 3. **Match Tracking**
- Maintains complete history of all match attempts
- Records acceptance/rejection decisions
- Stores user notes for rejected matches
- Prevents duplicate matching

### 4. **Smart Filtering**
- Filter people by name, profession, address, or parish
- Toggle between all businesses and suggested matches only
- Search functionality for quick navigation

## How to Use

### Accessing the Matcher

1. Launch the application
2. Select "Database Editor" from the game selection screen
3. Click the "Whites Matcher" tab in the navigation bar

### Generating Suggestions

1. **Set Minimum Confidence**:
   - Use the slider to adjust the minimum confidence threshold (0-100%)
   - Higher values show only strong matches
   - Lower values show more potential matches

2. **Set Maximum Suggestions**:
   - Choose how many suggestions to show per business entry (1-20)
   - Higher values provide more options but may include weaker matches

3. **Click "Generate Suggestions"**:
   - The system will analyze all unmatched businesses
   - Matching process may take a few seconds for large datasets
   - Results appear inline with each business entry

### Understanding Match Scores

**Confidence Levels:**
- **Excellent (80-100%)**: Green - Very likely to be correct
- **Good (60-79%)**: Orange - Probably correct, worth reviewing
- **Fair (40-59%)**: Red-orange - Possible match, needs verification
- **Weak (<40%)**: Gray - Unlikely match, use caution

**Match Factors:**
Each match is scored based on four factors (weighted):
- **Name (40%)**: Surname and first name similarity
- **Address (30%)**: Street address and house number matching
- **Profession (20%)**: Occupation/trade similarity
- **Parish (10%)**: Geographic parish matching

### Accepting/Rejecting Matches

#### Option 1: Using Suggestions (Recommended)

1. **Select a business entry** by clicking on it in the left panel
2. **Review the suggested matches** that appear inline
3. **Accept a match**:
   - Click the green "✓ Accept" button
   - Match is saved to the database
   - Business entry is removed from unmatched list
   - Player is removed from available candidates

4. **Reject a match**:
   - Click the red "✗ Reject" button
   - Optionally provide a reason in the dialog
   - Rejection is recorded to prevent re-suggesting

#### Option 2: Manual Drag-and-Drop

1. **Find a person** in the right panel (use search if needed)
2. **Click and drag** the person card
3. **Drop onto a business entry** in the left panel
4. **Confirm the match** in the dialog that appears
5. Match is calculated and saved automatically

### Best Practices

1. **Start with High Confidence**:
   - Begin with 60-70% minimum confidence
   - Accept obvious matches first
   - Lower threshold for remaining entries

2. **Review Match Reasons**:
   - Always read the match reason text
   - Check which factors contributed to the score
   - Be cautious of matches based on a single factor

3. **Use the Search**:
   - Filter people by parish when matching from specific areas
   - Search by profession for business-specific matches
   - Use address search when you know street names

4. **Check Context**:
   - Verify birth years make sense for the business activity
   - Ensure parishes align geographically
   - Consider profession compatibility

5. **Take Breaks**:
   - Matching is mentally intensive
   - Save progress by accepting matches as you go
   - Return later to continue with fresh eyes

## Data Structure

### Whites Directory Entry Fields
- **Name**: Business owner/proprietor name
- **Business Name**: Trading name (if different)
- **Business Type**: Category (e.g., "Inn", "Cutler", "Grocer")
- **Profession**: Occupation/trade
- **Street Address**: Full street address
- **District**: Area within Sheffield
- **Parish**: Civil or ecclesiastical parish
- **Year**: Directory publication year

### Sheffield People Fields
- **Name**: Full name
- **Birth Year**: Year of birth (helps verify age appropriateness)
- **Profession**: Occupation from census data
- **Street Address**: Address from genealogy/census records
- **Civil Parish**: Administrative parish
- **Ecclesiastical Parish**: Church parish

## Matching Algorithm Details

### Name Matching (40% weight)
- Exact surname match: 70 points
- Partial surname match: 50 points
- First name match: +30 points
- Initial match: +15 points

### Address Matching (30% weight)
- Exact address match: 100 points
- House number + street match: 100 points
- Street name only: 50 points

### Profession Matching (20% weight)
- Exact match: 100 points
- Partial/contains match: 70 points
- Synonym match (e.g., "innkeeper" ↔ "publican"): 80 points

### Parish Matching (10% weight)
- Exact match: 100 points
- Partial match: 70 points
- Normalized variant match (e.g., "Brightside Bierlow" ↔ "Brightside"): 100 points

## Database Tables

The matcher uses three main tables:

### sheffield_whites_entries
Stores all Whites Directory business listings with matching metadata.

### sheffield_whites_match_history
Records all match attempts, decisions, and user notes for audit trail.

### sheffield_whites_match_rules
Stores user-defined matching rules (future feature for customization).

## Statistics Dashboard

The stats panel shows:
- **Total Entries**: All businesses in Whites Directory
- **Matched**: Successfully matched businesses
- **Unmatched**: Remaining businesses to review
- **Suggestions**: Number of entries with auto-generated matches

## Keyboard Shortcuts

(Future enhancement - currently mouse-driven interface)

## Troubleshooting

### No suggestions appearing
- Lower the minimum confidence threshold
- Increase max suggestions per entry
- Check that unmatched entries exist
- Verify player candidates are available

### Matches seem incorrect
- Review the match factors breakdown
- Check birth years for reasonableness
- Verify parish/address alignment
- Use manual matching instead

### Performance issues
- Reduce max suggestions per entry
- Filter to specific areas/parishes
- Process in smaller batches
- Restart application if needed

## API Commands (Advanced)

For programmatic access:

```rust
// Get unmatched Whites entries
whites_get_unmatched_entries() -> Vec<WhitesEntry>

// Get available player candidates
whites_get_player_candidates() -> Vec<PlayerCandidate>

// Generate match suggestions
whites_generate_suggestions(min_confidence: f32, max_suggestions: usize) -> Vec<WhitesMatchSuggestion>

// Accept a match
whites_accept_match(whites_entry_id: String, player_id: String) -> Result<()>

// Reject a match
whites_reject_match(whites_entry_id: String, player_id: String, notes: Option<String>) -> Result<()>

// Get statistics
whites_get_stats() -> WhitesMatchStats
```

## Future Enhancements

Planned features:
- Bulk matching operations
- Export match report to CSV
- Undo/redo functionality
- Custom matching rules editor
- Machine learning suggestions
- Batch import from Whites Directory CSV files

## Credits

Matching algorithm inspired by genealogy record matching systems. Combines elements of Levenshtein distance, fuzzy string matching, and weighted scoring approaches.

---

**Last Updated**: February 2026
**Version**: 1.0.0
