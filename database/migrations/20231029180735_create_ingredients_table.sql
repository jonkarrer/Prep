CREATE TABLE ingredients (
    ingredient_id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    recipe_id CHAR(36) NOT NULL,
    ingredient_name VARCHAR(255) NOT NULL,
    amount FLOAT,
    unit VARCHAR(60)
)