const fs = require("fs");

let apiKey = "apiKey=76cc1ddbdebf4016ac72414e263c4ef3";
let numberOfRecipes = 100;
let random_endpoint = `https://api.spoonacular.com/recipes/random?${apiKey}&number=${numberOfRecipes}`;

let ids = ["1", "2", "3"].toString();
let bulk_endpoint = `https://api.spoonacular.com/recipes/informationBulk?${apiKey}&ids=${ids}`;

let i = 50;
let allRecipes = [];

try {
  while (i > 0) {
    let res = await fetch(random_endpoint);
    let data = await res.json();

    let recipes = data.recipes;
    allRecipes.push(...recipes);

    Bun.sleep(1000);
    console.log("i", i);
    i -= 1;
  }

  fs.writeFile(
    "random_recipes.json",
    JSON.stringify(allRecipes, null, 2),
    (err) => {
      if (err) {
        console.error("An error occurred:", err);
        return;
      }
      console.log("Data saved, no error");
    }
  );
} catch {
  fs.writeFile(
    "random_recipes.json",
    JSON.stringify(allRecipes, null, 2),
    (err) => {
      if (err) {
        console.error("An error occurred:", err);
        return;
      }
      console.log("Caught error, data saved");
    }
  );
}
