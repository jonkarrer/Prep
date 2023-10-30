CREATE TABLE directions (
    direction_id INT PRIMARY KEY AUTO_INCREMENT,
    recipe_id CHAR(36) NOT NULL,
    info VARCHAR(255) NOT NULL,
    step_order SMALLINT NOT NULL
)