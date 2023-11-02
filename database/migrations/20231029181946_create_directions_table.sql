CREATE TABLE directions (
    direction_id INT PRIMARY KEY AUTO_INCREMENT,
    recipe_id CHAR(36) NOT NULL,
    direction_info TEXT NOT NULL,
    step_order SMALLINT NOT NULL
)