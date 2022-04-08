-- Add up migration script here
CREATE TABLE puzzles (
    id text PRIMARY KEY NOT NULL,
    puzzle text UNIQUE NOT NULL,
    solution text NOT NULL,
    num_clues smallint NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
