use std::fs;
use text_io::scan;

fn read_input(name: &str) -> Vec<[i32; 5]> {
    let buf = fs::read_to_string(name).unwrap();
    let mut ingredients = Vec::new();
    let lines = buf.trim_end().split("\n");
    for l in lines {
        let name: String;
        let capacity: i32;
        let durability: i32;
        let flavor: i32;
        let texture: i32;
        let calories: i32;

        scan!(l.bytes() => "{}: capacity {}, durability {}, flavor {}, texture {}, calories {}", name, capacity, durability, flavor, texture, calories);
        ingredients.push([capacity, durability, flavor, texture, calories]);
    }

    ingredients
}

fn inc(v: &mut Vec<u32>) {
    let len = v.len();
    let mut found = false;
    while !found {
        v[0] += 1;

        for i in 0..len - 2 {
            if v[i] > 100 {
                v[i + 1] += 1;
                v[i] = 0;
            }
        }
        let rest = 100i64 - (v[0..len - 1].iter().sum::<u32>() as i64);
        if rest >= 0 {
            found = true;
        }
        v[len - 1] = rest as u32;
    }
}

fn score(recipe: &Vec<u32>, ingredients: &Vec<[i32; 5]>) -> i32 {
    if recipe.len() != ingredients.len() {
        panic!("OOps");
    }

    let mut vals = vec![0, 0, 0, 0];
    for i in 0..recipe.len() {
        vals = vals
            .iter()
            .zip(ingredients[i])
            .map(|(a, b)| a + recipe[i] as i32 * b)
            .collect();
    }

    vals = vals.iter().map(|&a| if a >= 0 { a } else { 0 }).collect();
    vals.iter().product()
}

fn calories(recipe: &Vec<u32>, ingredients: &Vec<[i32; 5]>) -> i32 {
    if recipe.len() != ingredients.len() {
        panic!("OOps");
    }

    let mut calories = 0;
    for i in 0..recipe.len() {
        calories += ingredients[i][4] * recipe[i] as i32;
    }

    calories
}

fn solve(ingredients: Vec<[i32; 5]>) {
    let mut recipe = vec![0; ingredients.len()];

    let mut best = 0;
    let mut best_500 = 0;
    while recipe[ingredients.len() - 2] != 100 {
        inc(&mut recipe);
        let score = score(&recipe, &ingredients);
        best = best.max(score);
        if calories(&recipe, &ingredients) == 500 {
            best_500 = best_500.max(score);
        }
    }
    println!("{best}");
    println!("{best_500}");
}

fn main() {
    let ingredients = read_input("input.txt");
    solve(ingredients);
}
