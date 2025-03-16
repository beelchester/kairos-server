CREATE TABLE projects (
    project_id UUID PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    project_name VARCHAR(255) NOT NULL,
    colour VARCHAR(50) NOT NULL,
    deadline TIMESTAMP WITH TIME ZONE,
    priority INTEGER
);

CREATE TABLE sessions (
    session_id UUID PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES projects(project_id) ON DELETE SET NULL,
    started_at TIMESTAMP WITH TIME ZONE NOT NULL,
    ended_at TIMESTAMP WITH TIME ZONE,
    duration INTEGER NOT NULL
);

CREATE INDEX idx_sessions_started_at ON sessions (started_at);
CREATE INDEX idx_sessions_id ON sessions (session_id);
CREATE INDEX idx_sessions_user_id ON sessions (user_id);
CREATE INDEX idx_sessions_project_id ON sessions (project_id);

