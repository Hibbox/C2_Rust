CREATE TABLE task (
    task_id UUID PRIMARY KEY, -- généré côté Rust
    task_type TEXT NOT NULL,
    task_options JSONB NOT NULL
);