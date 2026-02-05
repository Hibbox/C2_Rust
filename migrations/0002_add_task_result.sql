CREATE TABLE task_results (
    result_id UUID PRIMARY KEY,  -- PAS de DEFAULT, généré côté Rust
    task_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL,
    output JSONB NOT NULL,
    completed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    execution INTEGER,
    FOREIGN KEY (task_id) REFERENCES task(task_id) ON DELETE CASCADE
);
CREATE INDEX idx_task_created_at ON task(created_at DESC);