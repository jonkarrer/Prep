CREATE TABLE ingredients (
    ingredient_id INT PRIMARY KEY AUTO_INCREMENT,
    recipe_id CHAR(36) NOT NULL,
    name VARCHAR(60) NOT NULL,
    amount FLOAT,
    unit VARCHAR(60)
)