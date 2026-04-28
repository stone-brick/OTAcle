-- OTAcle Database Initialization Script
-- This script is automatically executed by H2 on startup

-- Create tables (JPA will auto-create, but this is for reference)

-- Projects table
CREATE TABLE IF NOT EXISTS projects (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    project_path VARCHAR(500),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

-- Actions table
CREATE TABLE IF NOT EXISTS actions (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    project_id BIGINT,
    action_type VARCHAR(50),
    config TEXT,
    variables TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Decision logs table
CREATE TABLE IF NOT EXISTS decision_logs (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    step INTEGER NOT NULL,
    custom_data TEXT,
    project_id BIGINT,
    created_at TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Image frames table
CREATE TABLE IF NOT EXISTS image_frames (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    frame_id INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    timestamp BIGINT NOT NULL,
    crop_blocks TEXT,
    project_id BIGINT,
    created_at TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Action commands table
CREATE TABLE IF NOT EXISTS action_commands (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    execute_flags TEXT,
    params TEXT,
    project_id BIGINT,
    executed_at TIMESTAMP,
    created_at TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_actions_project_id ON actions(project_id);
CREATE INDEX IF NOT EXISTS idx_decision_logs_project_id ON decision_logs(project_id);
CREATE INDEX IF NOT EXISTS idx_decision_logs_step ON decision_logs(step);
CREATE INDEX IF NOT EXISTS idx_image_frames_project_id ON image_frames(project_id);
CREATE INDEX IF NOT EXISTS idx_image_frames_frame_id ON image_frames(frame_id);
CREATE INDEX IF NOT EXISTS idx_action_commands_project_id ON action_commands(project_id);

-- Insert sample data (optional)
INSERT INTO projects (name, description, project_path, created_at, updated_at)
VALUES ('Demo Project', 'Sample project for testing', '/path/to/demo', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON DUPLICATE KEY UPDATE name = name;
