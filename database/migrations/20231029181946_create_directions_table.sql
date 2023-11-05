CREATE TABLE directions (
    direction_id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    recipe_id CHAR(36) NOT NULL,
    direction_details TEXT NOT NULL,
    step_order SMALLINT UNSIGNED NOT NULL
)