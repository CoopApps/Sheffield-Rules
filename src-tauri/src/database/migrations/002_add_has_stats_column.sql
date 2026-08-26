-- Migration: Add has_stats column to track player stat assignment
ALTER TABLE sheffield_players ADD COLUMN has_stats BOOLEAN DEFAULT 0;
