CREATE TABLE task_history (
    id SERIAL PRIMARY KEY,
    task_id UUID NOT NULL REFERENCES task(task_id),
    task_type VARCHAR(255) NOT NULL,
    task_options JSONB NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_task_history_task_id ON task_history(task_id);
CREATE INDEX idx_task_history_created_at ON task_history(created_at DESC);