-- Create sheffield_challenge_invitations table
CREATE TABLE IF NOT EXISTS sheffield_challenge_invitations (
    id TEXT PRIMARY KEY,
    sender_club_id TEXT NOT NULL,
    recipient_club_id TEXT NOT NULL,
    sent_date TEXT NOT NULL,
    response_date TEXT NOT NULL,
    proposed_match_date TEXT NOT NULL,
    match_type TEXT NOT NULL,
    venue TEXT NOT NULL,
    stakes TEXT NOT NULL,
    tone TEXT NOT NULL,
    rules_type TEXT NOT NULL,
    match_duration INTEGER NOT NULL,
    status TEXT NOT NULL,
    response_text TEXT,
    acceptance_likelihood INTEGER,
    decline_reason TEXT,
    sent_news_id TEXT,
    response_news_id TEXT,
    scheduled_match_id TEXT,
    FOREIGN KEY (sender_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (recipient_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (scheduled_match_id) REFERENCES sheffield_matches(id)
);

-- Create sheffield_news_items table
CREATE TABLE IF NOT EXISTS sheffield_news_items (
    id TEXT PRIMARY KEY,
    headline TEXT NOT NULL,
    article_type TEXT NOT NULL,
    publish_date TEXT NOT NULL,
    body_text TEXT NOT NULL,
    is_read INTEGER NOT NULL DEFAULT 0,
    is_important INTEGER NOT NULL DEFAULT 0,
    requires_action INTEGER NOT NULL DEFAULT 0,
    related_club_ids TEXT,
    related_player_ids TEXT,
    related_match_id TEXT,
    related_invitation_id TEXT,
    has_action_button INTEGER NOT NULL DEFAULT 0,
    action_button_text TEXT,
    action_type TEXT,
    action_data TEXT,
    FOREIGN KEY (related_match_id) REFERENCES sheffield_matches(id),
    FOREIGN KEY (related_invitation_id) REFERENCES sheffield_challenge_invitations(id)
);
