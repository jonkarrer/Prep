CREATE TABLE user_sessions (
    id CHAR(36) PRIMARY KEY,
    created_at BIGINT UNSIGNED NOT NULL,
    expires_at BIGINT UNSIGNED NOT NULL,
    user_identity VARCHAR(255) NOT NULL
)