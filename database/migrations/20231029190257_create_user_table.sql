CREATE TABLE users (
    row_id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    user_id CHAR(36) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    credential_id CHAR(36) NOT NULL,
    user_name VARCHAR(255) NOT NULL,
    password_reset_token VARCHAR(255) DEFAULT NULL,
    password_reset_expiry TIMESTAMP DEFAULT NULL,
    last_login TIMESTAMP DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    profile_pic_url VARCHAR(255) DEFAULT NULL,
    role ENUM('user', 'admin', 'editor') DEFAULT 'user'
)