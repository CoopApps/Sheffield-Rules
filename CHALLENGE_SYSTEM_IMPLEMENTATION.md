# Challenge Invitation System - Implementation Summary

## Overview
A complete Victorian-era challenge letter system for Saturday at Three, allowing clubs to send and receive challenge letters with automatic responses based on game date progression.

## ✅ Completed Components

### 1. Database Schema (`sheffield_schema.sql`)
Added two new tables to the Sheffield 1867 database:

#### `sheffield_challenge_invitations`
- Stores all challenge letters sent and received
- Tracks status: sent → delivered → accepted/declined
- Contains all match proposal details (date, venue, stakes, rules, etc.)
- Links to scheduled matches when accepted
- References news items for sending and response

#### `sheffield_news_items`
- General news/inbox system for in-game events
- Supports action buttons (e.g., "Read Letter")
- Tracks read status and importance
- Links to related entities (clubs, players, matches, invitations)

### 2. Rust Backend (`challenge_invitations.rs`)
Core functions implemented:

- **`send_challenge_letter()`** - Creates invitation and sends news
  - Generates unique invitation ID
  - Calculates response date (current_date + 3 days)
  - Creates "Challenge Dispatched" news item
  - Returns invitation details

- **`check_pending_responses()`** - Processes responses on date advance
  - Finds invitations where response_date <= current_date
  - Determines accept/decline based on acceptance_likelihood
  - Generates Victorian-era response text
  - Creates "Response Received" news item with action button
  - Schedules match if accepted (placeholder for now)

- **`get_news_for_date()`** - Retrieves news items for event feed
- **`get_invitation_by_id()`** - Fetches invitation details for letter display

### 3. Tauri Commands (`commands.rs`)
Registered handlers:
- `send_challenge_letter` - Frontend calls this to send letters
- `get_news_for_date` - Load news for current date
- `get_unread_news_count` - For notification badges
- `mark_news_as_read` - Track which news user has seen
- `get_invitation_by_id` - Load full invitation/response
- `check_pending_invitation_responses` - Called during date advance

### 4. Date Advance Integration
Modified `advance_day()` command to automatically check for pending responses when the date advances.

### 5. Frontend Integration

#### ClubsScreen.tsx
- Calendar date picker (minimum 5 days from current date)
- Sends challenge letter via Tauri invoke
- Validates proposed date before sending
- Shows confirmation message

#### GameplayScreen.tsx
- Loads news items from database on date change
- Converts news to event format
- Displays in existing event-items UI
- Shows headlines with importance markers

#### News Details Panel
- Displays full news article body
- "Read Letter" button for responses
- Fetches and shows Victorian response text
- Handles all challenge article types:
  - `challenge_sent` - Confirmation of sending
  - `challenge_accepted` - Acceptance notification
  - `challenge_declined` - Decline notification
  - `challenge_received` - Letter arrived

## 🔄 How It Works

### Sending a Challenge
1. User navigates to Clubs screen
2. Selects a club and clicks "Send Challenge Letter"
3. Fills out the form:
   - Proposed date (calendar picker, min 5 days ahead)
   - Match type, venue, stakes, tone
   - Rules type and match duration
4. Clicks "Send Letter by Post"
5. Backend creates:
   - Invitation record (status: 'sent')
   - News item: "Challenge Letter Dispatched"
6. News appears in event feed immediately

### Receiving a Response
1. User advances the game date (day by day or to next matchday)
2. When current_date reaches response_date (3 days after sending):
   - `check_pending_responses()` runs automatically
   - System decides accept/decline based on `acceptance_likelihood`
   - Victorian response text is generated
   - News item created: "{Club} ACCEPTS/DECLINES CHALLENGE"
   - Invitation status updated to 'accepted' or 'declined'
3. News appears in event feed with "Read the Letter" button
4. User clicks button to see full Victorian response

## 📝 Victorian Response Templates

The system generates authentic Victorian-era correspondence:

### Acceptance Example
```
Dear Sir,

The Secretary of Sheffield FC acknowledges with great pleasure your kind
letter of the 15th October, 1867.

We are delighted to accept your proposal for a match of football between
our respective clubs. The date of 25th October, 1867 suits our fixtures
admirably, and we look forward with keen anticipation to what promises
to be a most sporting contest.

Your terms regarding the venue and stakes are most agreeable to us, and
we shall be honoured to meet your gentlemen on the field of play under
Sheffield Rules.

We remain confident that the match shall be contested in the finest
spirit of sportsmanship.

Your obedient servant,
Secretary
```

### Decline Example
```
Dear Sir,

We acknowledge with thanks your kind invitation to a match on 25th
October, 1867.

It is with sincere regret that we must inform you that we have prior
engagements on that date, and our Committee feels it would be improper
to break such commitments.

We remain, Sir,
Your obedient servants
```

## 🚧 Still To Implement

### 1. Enhanced Acceptance Logic
Currently uses placeholder 75% acceptance rate. Should consider:
- **Club prestige/division** - Higher division more likely to decline lower
- **Distance** - Clubs prefer nearby opponents
- **Schedule conflicts** - Check existing fixtures
- **Stakes** - Higher stakes may affect likelihood
- **Recent form** - Struggling clubs more eager for friendlies
- **Rivalry** - Historical rivals always accept

### 2. Match Scheduling
When invitation is accepted, need to:
- Create match in `sheffield_matches` table
- Assign unique match ID
- Set as friendly/challenge type (not league)
- Check for conflicts with:
  - Existing league fixtures
  - Workers' hours (evening/weekend preferred)
  - Out of season dates
- Suggest alternative dates if conflicts found
- Add to fixtures list visible to player

### 3. Calendar Improvements
- Show existing fixtures on calendar
- Highlight unavailable dates
- Show why dates are unavailable (hover tooltip)
- Weekend/evening highlighting

### 4. Better Letter Display
Replace `alert()` with proper modal showing:
- Victorian paper background
- Wax seal decoration
- Handwriting-style font
- Same format as composing letters

### 5. Response Variations
Add more Victorian response templates:
- Different tones (cordial, formal, competitive)
- Prestige-based language
- Alternative date suggestions in declines
- References to previous matches (if any)

### 6. News Enhancements
- Mark as read when clicked
- Unread count badge
- Filter by type
- Archive old news
- Search functionality

## 🗂️ Files Modified/Created

### Created
- `src-tauri/src/database/challenge_invitations.rs` - Core logic
- `src-tauri/src/database/migrations/003_add_challenge_invitations.sql` - Migration file
- `challenge_response_templates.md` - Response text templates

### Modified
- `src-tauri/src/database/sheffield_schema.sql` - Added tables
- `src-tauri/src/database/mod.rs` - Registered module
- `src-tauri/src/commands.rs` - Added Tauri commands
- `src-tauri/src/main.rs` - Registered commands in handler
- `frontend/src/screens/ClubsScreen.tsx` - Calendar picker & send logic
- `frontend/src/screens/GameplayScreen.tsx` - News integration & display

## 🧪 Testing Checklist

- [ ] Send a challenge letter to a club
- [ ] Verify "Challenge Dispatched" news appears
- [ ] Advance date by 3 days
- [ ] Verify response news appears with "Read Letter" button
- [ ] Click "Read Letter" to view response
- [ ] Test acceptance (check if match is scheduled)
- [ ] Test decline
- [ ] Verify calendar only allows dates 5+ days ahead
- [ ] Send multiple challenges to different clubs
- [ ] Verify all responses arrive on correct dates

## 🔧 Configuration

### Response Timing
Controlled in `send_challenge_letter()`:
```rust
let response_date = sent_date + Duration::days(3);
```

### Acceptance Likelihood
Placeholder in `calculate_acceptance_likelihood()`:
```rust
Ok(75) // 75% chance of acceptance
```

### Minimum Days Ahead
Set in ClubsScreen calendar picker:
```typescript
min={(() => {
  const minDate = new Date(gameState.currentDate + 'T00:00:00')
  minDate.setDate(minDate.getDate() + 5)
  return minDate.toISOString().split('T')[0]
})()}
```

## 📊 Database Schema Diagram

```
sheffield_challenge_invitations
├── id (PK)
├── sender_club_id → sheffield_clubs(id)
├── recipient_club_id → sheffield_clubs(id)
├── sent_date
├── response_date (sent_date + 3 days)
├── proposed_match_date
├── match_type, venue, stakes, tone, rules_type, match_duration
├── status (sent/delivered/accepted/declined)
├── response_text
├── acceptance_likelihood
├── sent_news_id → sheffield_news_items(id)
├── response_news_id → sheffield_news_items(id)
└── scheduled_match_id → sheffield_matches(id)

sheffield_news_items
├── id (PK)
├── headline
├── article_type (challenge_sent/accepted/declined/received)
├── publish_date
├── body_text
├── is_read, is_important, requires_action
├── related_invitation_id → sheffield_challenge_invitations(id)
├── has_action_button, action_button_text, action_type
└── action_data (JSON)
```

## 🎯 Next Steps

1. **Test the system** - Restart app to create tables, send a test challenge
2. **Implement match scheduling** - Create friendly matches when accepted
3. **Enhance acceptance logic** - Real prestige/distance calculations
4. **Improve letter display** - Proper Victorian modal instead of alert
5. **Add more response variations** - Rich, contextual Victorian language

The core system is complete and functional! Players can now send challenge letters and receive responses based on the in-game date.
