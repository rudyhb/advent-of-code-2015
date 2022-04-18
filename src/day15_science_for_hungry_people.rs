use std::ops::AddAssign;

pub(crate) fn run() {
    let _input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    let _input = _get_input();

    let ingredients: Vec<Ingredient> = _input.split('\n').map(|line| line.into()).collect();

    let mut recipe = Recipe::from_iter(ingredients);

    recipe.optimize(None);
    println!("best score: {}", recipe.get_score());

    recipe.optimize(Some(500));
    println!("best score for 500 calories: {}", recipe.get_score());
}

struct Recipe {
    ingredients: Vec<Ingredient>,
    ingredient_proportions: Vec<u8>
}

impl Recipe {
    pub(crate) fn get_score(&self) -> i32 {
        self.aggregate_properties(&self.ingredient_proportions[..]).score()
    }
    fn aggregate_properties(&self, vec: &[u8]) -> IngredientProperties {
        self.ingredients.iter().enumerate().fold(Default::default(), |mut sum, (index, next)| {
            sum += next.properties.multiply(vec[index]);
            sum
        })
    }
    fn get_combinations(sum: u8, len: usize) -> Vec<Vec<u8>> {
        if len == 1 {
            vec![vec![sum]]
        } else {
            (0..=sum)
                .map(|first| {
                    let mut combinations = Self::get_combinations(sum - first, len - 1);
                    for c in combinations.iter_mut() {
                        c.push(first);
                    }
                    combinations
                })
                .flat_map(|c| c)
                .collect()
        }
    }
    pub(crate) fn optimize(&mut self, calories: Option<i32>) {
        let combinations = Self::get_combinations(100, self.ingredients.len());
        println!("there are {} combinations", combinations.len());

        let mut best_guess = Default::default();
        let mut best_score = 0;
        for guess in combinations {
            let aggregate = self.aggregate_properties(&guess[..]);
            if let Some(calories) = calories {
                if calories != aggregate.calories {
                    continue;
                }
            }
            let score = aggregate.score();
            if  score > best_score {
                best_score = score;
                best_guess = guess.clone();
            }
        }

        self.ingredient_proportions = best_guess;
    }
}

impl FromIterator<Ingredient> for Recipe {
    fn from_iter<T: IntoIterator<Item=Ingredient>>(iter: T) -> Self {
        let ingredients: Vec<Ingredient> = iter.into_iter().collect();
        let len = ingredients.len() as u8;
        let mut ingredient_proportions = vec![100u8 / len; len as usize];
        ingredient_proportions[0] = 100u8 - (len - 1) * (100u8 / len);

        Self {
            ingredients,
            ingredient_proportions
        }
    }
}

struct IngredientProperties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[allow(unused)]
struct Ingredient {
    name: &'static str,
    properties: IngredientProperties,
}

impl Default for IngredientProperties {
    fn default() -> Self {
        Self {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }
}

impl IngredientProperties {
    pub(crate) fn multiply(&self, by: u8) -> Self {
        Self {
            capacity: self.capacity * by as i32,
            durability: self.durability * by as i32,
            flavor: self.flavor * by as i32,
            texture: self.texture * by as i32,
            calories: self.calories * by as i32,
        }
    }
    pub(crate) fn score(&self) -> i32 {
        let mut product = 1;
        product *= self.capacity.max(0);
        product *= self.durability.max(0);
        product *= self.flavor.max(0);
        product *= self.texture.max(0);

        product
    }
}

impl AddAssign for IngredientProperties {
    fn add_assign(&mut self, rhs: Self) {
        self.capacity += rhs.capacity;
        self.durability += rhs.durability;
        self.flavor += rhs.flavor;
        self.texture += rhs.texture;
        self.calories += rhs.calories;
    }
}

impl From<&'static str> for Ingredient {
    fn from(s: &'static str) -> Self {
        let mut words = s.split_whitespace();
        let name = words.next().unwrap().trim_end_matches(':');
        words.next();
        let capacity = words.next().unwrap().trim_end_matches(',').parse().unwrap();
        words.next();
        let durability = words.next().unwrap().trim_end_matches(',').parse().unwrap();
        words.next();
        let flavor = words.next().unwrap().trim_end_matches(',').parse().unwrap();
        words.next();
        let texture = words.next().unwrap().trim_end_matches(',').parse().unwrap();
        words.next();
        let calories = words.next().unwrap().trim_end_matches(',').parse().unwrap();
        Self {
            name,
            properties: IngredientProperties {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            },
        }
    }
}

fn _get_input() -> &'static str {
    "Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5
Candy: capacity 0, durability 5, flavor -1, texture 0, calories 8
Butterscotch: capacity -1, durability 0, flavor 5, texture 0, calories 6
Sugar: capacity 0, durability 0, flavor -2, texture 2, calories 1"
}