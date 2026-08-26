-- Challenge Invitation System
-- Manages challenge letters sent between clubs with date-based responses

-- Challenge invitations table: Stores all challenge letters sent and received
CREATE TABLE IF NOT EXISTS sheffield_challenge_invitations (
    id TEXT PRIMARY KEY,
    sender_club_id TEXT NOT NULL,
    recipient_club_id TEXT NOT NULL,

    -- Dates
    sent_date TEXT NOT NULL,              -- When the letter was sent
    response_date TEXT NOT NULL,          -- When response will arrive (sent_date + 3-4 days)
    proposed_match_date TEXT NOT NULL,    -- Proposed date for the match

    -- Match details from the letter
    match_type TEXT NOT NULL,             -- 'friendly', 'practice', 'challenge'
    venue TEXT NOT NULL,                  -- 'home', 'away', 'neutral'
    stakes TEXT NOT NULL,                 -- 'honor', 'small', 'medium', 'trophy', 'silver-cup', 'dinner', 'charity'
    tone TEXT NOT NULL,                   -- 'cordial', 'formal', 'respectful', 'confident', 'humble', 'competitive', 'bold'
    rules_type TEXT NOT NULL,             -- 'sheffield', 'fa', 'rugby'
    match_duration INTEGER NOT NULL,      -- 60, 80, 90, 120 (in minutes)

    -- Status tracking
    status TEXT NOT NULL DEFAULT 'sent',  -- 'sent', 'delivered', 'accepted', 'declined', 'expired'
    response_text TEXT,                   -- The Victorian-era response text
    acceptance_likelihood INTEGER,        -- 0-100 calculated when sent

    -- News integration
    sent_news_id TEXT,                    -- Reference to news story about sending
    response_news_id TEXT,                -- Reference to news story about receiving response

    -- Match scheduling (if accepted)
    scheduled_match_id TEXT,              -- Reference to the actual match created

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (sender_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (recipient_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (scheduled_match_id) REFERENCES sheffield_matches(id)
);

-- News/Inbox system for in-game events
CREATE TABLE IF NOT EXISTS sheffield_news_items (
    id TEXT PRIMARY KEY,
    headline TEXT NOT NULL,
    article_type TEXT NOT NULL,           -- 'challenge_sent', 'challenge_received', 'match_result', 'cup_announcement', etc.
    publish_date TEXT NOT NULL,           -- When this news appears
    body_text TEXT NOT NULL,              -- Main article text

    -- Status
    is_read BOOLEAN DEFAULT 0,
    is_important BOOLEAN DEFAULT 0,

    -- Related entities
    related_club_ids TEXT,                -- JSON array of club IDs mentioned
    related_player_ids TEXT,              -- JSON array of player IDs mentioned
    related_match_id TEXT,                -- If about a match
    related_invitation_id TEXT,           -- If about a challenge invitation

    -- Display
    has_action_button BOOLEAN DEFAULT 0,  -- Does this news have a button (e.g., "Read Letter")
    action_button_text TEXT,              -- Text for the button
    action_type TEXT,                     -- 'read_letter', 'view_match', etc.
    action_data TEXT,                     -- JSON data for the action

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (related_match_id) REFERENCES sheffield_matches(id),
    FOREIGN KEY (related_invitation_id) REFERENCES sheffield_challenge_invitations(id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_invitations_sender ON sheffield_challenge_invitations(sender_club_id);
CREATE INDEX IF NOT EXISTS idx_invitations_recipient ON sheffield_challenge_invitations(recipient_club_id);
CREATE INDEX IF NOT EXISTS idx_invitations_response_date ON sheffield_challenge_invitations(response_date);
CREATE INDEX IF NOT EXISTS idx_invitations_status ON sheffield_challenge_invitations(status);
CREATE INDEX IF NOT EXISTS idx_news_publish_date ON sheffield_news_items(publish_date);
CREATE INDEX IF NOT EXISTS idx_news_is_read ON sheffield_news_items(is_read);
CREATE INDEX IF NOT EXISTS idx_news_invitation ON sheffield_news_items(related_invitation_id);
