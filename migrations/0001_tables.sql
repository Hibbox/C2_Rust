CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS task (
    task_id UUID PRIMARY KEY,
    task_type VARCHAR(255) NOT NULL,
    task_options JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS task_result (
    result_id UUID PRIMARY KEY,
    task_id UUID NOT NULL,
    sucess BOOL NOT NULL,
    output JSONB NOT NULL,
    completed_at TIMESTAMP DEFAULT NOW(),
    execution INTEGER,
    FOREIGN KEY (task_id) REFERENCES task(task_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS task_history (
    task_id UUID NOT NULL,
    task_type VARCHAR(255) NOT NULL,
    task_options JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    sucess BOOL NOT NULL
);