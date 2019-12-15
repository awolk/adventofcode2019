use std::collections::{HashMap, HashSet};

// Parsing

#[derive(Debug)]
struct Item<'a> {
    name: &'a str,
    count: u64,
}

impl<'a> Item<'a> {
    fn from_str(s: &'a str) -> Result<Self, String> {
        let parts: Vec<_> = s.splitn(2, " ").collect();
        let count = parts[0]
            .parse()
            .map_err(|err| format!("invalid count: {}", err))?;
        let name = parts[1];
        Ok(Item { name, count })
    }
}

#[derive(Debug)]
struct Formula<'a> {
    inputs: Vec<Item<'a>>,
    output: Item<'a>,
}

impl<'a> Formula<'a> {
    fn from_str(s: &'a str) -> Result<Self, String> {
        let parts: Vec<&'a str> = s.splitn(2, " => ").collect();
        let input_str = parts[0];
        let output_str = parts[1];

        let inputs: Vec<Item<'a>> = input_str
            .split(", ")
            .map(|item| Item::from_str(item))
            .collect::<Result<Vec<Item<'a>>, String>>()
            .map_err(|err| format!("invalid input: {}", err))?;
        let output =
            Item::from_str(output_str).map_err(|err| format!("invalid output: {}", err))?;

        Ok(Formula { inputs, output })
    }
}

fn parse_formulas(input: &str) -> Result<Vec<Formula>, String> {
    input.lines().map(|line| Formula::from_str(line)).collect()
}

// Implementation

fn min_ore_for_fuel(formulas: &Vec<Formula>) -> u64 {
    // build table of formulas and graph of ingredients
    let mut formula_for = HashMap::new();
    let mut ingredients = HashMap::new(); // forward edges
    let mut creates = HashMap::new(); // reverse edges
    for formula in formulas {
        formula_for.insert(formula.output.name, formula);
        ingredients.insert(
            formula.output.name,
            formula
                .inputs
                .iter()
                .map(|item| item.name)
                .collect::<Vec<&str>>(),
        );
        for input in &formula.inputs {
            creates
                .entry(input.name)
                .or_insert_with(HashSet::new)
                .insert(formula.output.name);
        }
    }

    // perform a topological sort on the ingredients graph, and process required ingredients as the
    // sort progresses
    let mut needs = HashMap::new();
    needs.insert("FUEL", 1);
    let mut to_process = vec!["FUEL"];

    while !to_process.is_empty() {
        let processing = to_process.pop().unwrap();
        if processing == "ORE" {
            break;
        }

        // handle current chemical
        if let Some(count) = needs.remove(processing) {
            let formula = formula_for.get(processing).unwrap();
            let applications = (count as f64 / formula.output.count as f64).ceil() as u64;
            for input in &formula.inputs {
                *needs.entry(input.name).or_insert(0) += applications * input.count;
            }
        }

        // handle next steps of topological sort
        let ingredients_of = ingredients.remove(processing).unwrap();
        for ingredient in ingredients_of {
            let ingredient_used_for = creates.get_mut(ingredient).unwrap();
            ingredient_used_for.remove(processing);

            if ingredient_used_for.is_empty() {
                creates.remove(ingredient);
                to_process.push(ingredient);
            }
        }
    }
    // validate that graph has no cycles
    assert!(ingredients.is_empty());
    assert!(creates.is_empty());

    *needs.get("ORE").unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let formulas = parse_formulas(input).expect("failed to parse formulas");
    println!(
        "Part 1: minimum ORE for 1 FUEL = {:}",
        min_ore_for_fuel(&formulas)
    );
}
