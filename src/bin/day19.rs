use advent_of_code::iterator_to_string;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};

const INPUT: &str = advent_of_code::get_input!();

type RecipePart = (Material, usize);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: usize,
    robot_recipes: [Vec<RecipePart>; 4],
    max_materials_for_any_recipes: [usize; 4],
}

impl Blueprint {
    fn new(id: usize, robot_recipes: [Vec<RecipePart>; 4]) -> Self {
        let max_materials_for_any_recipes = get_max_materials(&robot_recipes);
        Self {
            id,
            robot_recipes,
            max_materials_for_any_recipes
        }
    }
}

// Finds the maximum of any material needed in any recipe
fn get_max_materials(robot_recipes: &[Vec<RecipePart>; 4]) -> [usize; 4] {
    robot_recipes
        .iter()
        .fold([0, 0, 0, usize::MAX], |mut maxs, recipe| {
            for &(material, amount) in recipe {
                let i = material as usize;
                maxs[i] = std::cmp::max(maxs[i], amount);
            }
            maxs
        })
}

#[derive(Copy, Clone)]
struct State {
    time_left: usize,
    robots: [usize; 4],
    materials: [usize; 4],
}

impl State {
    // start with no materials and one ore robot
    fn new(time_left: usize) -> Self {
        Self {
            time_left,
            robots: [1, 0, 0, 0],
            materials: [0, 0, 0, 0],
        }
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("time_left", &self.time_left)
            .field("robots", &iterator_to_string(self.robots.iter(), ","))
            .field("materials", &iterator_to_string(self.materials.iter(), ","))
            .finish()
    }
}

// determine if a robot can be built
fn robot_can_be_build_from(materials: &[usize; 4], robot_recipe: &[RecipePart]) -> bool {
    robot_recipe.iter().all(|&(material, amount)| materials[material as usize] >= amount)
}

impl State {
    fn can_build_robot(&self,blueprint: &Blueprint, robot_type: Material) -> bool {
        let robot_type = robot_type as usize;
        let robot_recipe = &blueprint.robot_recipes[robot_type];
        // Don't build this robot if we are already generating the maximum amount of the robot's material per minute
        let maxed_out = self.robots[robot_type] >= blueprint.max_materials_for_any_recipes[robot_type];
        !maxed_out && robot_can_be_build_from(&self.materials, robot_recipe)
    }

    // Updates self to deduct materials and add one robot of the specified type
    // We assume that the robot can be built
    fn build_robot(&mut self, robot_type: Material, blueprint: &Blueprint) {
        let robot_type = robot_type as usize;
        for &(material, amount) in &blueprint.robot_recipes[robot_type] {
            self.materials[material as usize] -= amount;
        }
        self.robots[robot_type] += 1;
    }

    fn collect_materials(&mut self) {
        (0..4).for_each(|i| self.materials[i] += self.robots[i]);
    }

    fn collect_materials_for(&mut self, minutes: usize) {
        (0..4).for_each(|i| self.materials[i] += self.robots[i] * minutes);
    }
}

fn max_geodes(blueprint: &Blueprint, time_left: usize) -> usize {
    // calculate the maximum amount for every type of bot so that the creation of a new bot of any type is never bottlenecked
    // it doesn't make sense to build more bots than that maximum if the resources a bot type generates are
    // enough to cover that type (ore, clay, obsidian) cost for any possible bot (per question, you can only build 1 bot per turn)
    // for geode bots, there is no logical maximum amount

    let mut max_geodes = 0;

    let mut q = std::collections::VecDeque::new();
    q.push_back(State::new(time_left));

    while let Some(state) = q.pop_front() {
        for robot_type in [Material::Ore, Material::Clay, Material::Obsidian, Material::Geode] {
            let robot_type_index = robot_type as usize;

            // if we already have enough of this bot type, skip
            if state.robots[robot_type_index] >= blueprint.max_materials_for_any_recipes[robot_type_index] { continue; }
            
            // Find the limiting resource type for the robot_recipe
            let wait_time = blueprint.robot_recipes[robot_type_index].iter().map(|(material, cost)| {
                let material_idx = *material as usize;
                match *cost { 
                    cost if cost <= state.materials[material_idx] => 0,
                    // no bot made yet in order to gather the required material
                    // we can't build it (it takes more than max_time to build it).
                    _ if state.robots[material_idx] == 0 => state.time_left + 1,
                    _ => (cost - state.materials[material_idx] + (state.robots[material_idx] - 1)) / state.robots[material_idx],
                }
            })
            .max()
            .unwrap() + 1;
            // the + 1 is so the built bot has the chance to do something, it merely being built is not enough

            // if that choice would cause the time limit be to exceeded, skip
            let Some(new_time_left) = state.time_left.checked_sub(wait_time) else {
                continue;
            };
            
            let mut new_state = State {
                robots: state.robots,
                materials: state.materials,
                time_left: new_time_left,
            };

            // gather ores with previously available bots
            new_state.collect_materials_for(wait_time);

            // increase bot type for the bot we just built
            // let mut new_state = state.clone();
            new_state.build_robot(robot_type, blueprint);
            
            // extra optimization:
            // if we theoretically only built geode bots every turn, and we still don't beat the maximum, skip
            if optimistic_best(&new_state, Material::Geode) < max_geodes { continue; }
            q.push_back(new_state)
        }

        let geode_idx = Material::Geode as usize;
        let geodes_for_this_state = state.materials[geode_idx] + state.robots[geode_idx] * state.time_left;
        max_geodes = max_geodes.max(geodes_for_this_state);
    }

    max_geodes
}

// Determines the optimistic best material production from this state, assuming that
// we can build a robot for that material every turn from now until the end
fn optimistic_best(state: &State, material: Material) -> usize {
    let material = material as usize;

    if state.time_left == 0 { return state.materials[material]; }

    // The material that we already have...
    // plus the material that will be generated by the existing robots...
    // plus the optimistic assumption that one new robot will be added every turn (1 + 2 + ... + i) = i * (i + 1) / 2
    state.materials[material] + state.robots[material] * state.time_left + (state.time_left * (state.time_left - 1)) / 2 
}

fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(line_ending, parse_blueprint)(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(complete::u64, |x| x as usize)(input)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(
        tag("Blueprint "),
        parse_usize,
        tag(": ")
    )(input)?;
    let (input, ore_ore) = delimited(
        tag("Each ore robot costs "),
        parse_usize,
        tag(" ore. "),
    )(input)?;
    let (input, clay_ore) = delimited(
        tag("Each clay robot costs "),
        parse_usize,
        tag(" ore. "),
    )(input)?;
    let (input, (obsidian_ore, obsidian_clay)) = separated_pair(
        preceded(tag("Each obsidian robot costs "), parse_usize),
        tag(" ore and "),
        terminated(parse_usize, tag(" clay. ")),
    )(input)?;
    let (input, (geode_ore, geode_obsidian)) = separated_pair(
        preceded(tag("Each geode robot costs "), parse_usize),
        tag(" ore and "),
        terminated(parse_usize, tag(" obsidian.")),
    )(input)?;

    Ok((
        input,
        Blueprint::new(
            id,
            [
                vec![(Material::Ore, ore_ore)],
                vec![(Material::Ore, clay_ore)],
                vec![
                    (Material::Ore, obsidian_ore),
                    (Material::Clay, obsidian_clay),
                ],
                vec![
                    (Material::Ore, geode_ore),
                    (Material::Obsidian, geode_obsidian),
                ],
            ],
        ),
    ))
}

fn main() {
    let (input, blueprints) = parse_blueprints(INPUT).unwrap();
    let blueprints_quality_levels_sum = blueprints.par_iter()
        .map(|blueprint|
            blueprint.id * max_geodes(blueprint, 24)
        )
        .sum::<usize>();

    println!("The sum of the quality levels of each blueprints is is {}", blueprints_quality_levels_sum);

    let blueprints_max_geodes_prod = blueprints
            .par_iter()
            .take(3)
            .map(|blueprint| max_geodes(blueprint, 32))
            .product::<usize>();
    
    println!("The product of the maximum number of geodes that can be produced by first 3 blueprints is {}", blueprints_max_geodes_prod);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn parsing_test() {
        assert_eq!(
            parse_blueprints(TEST_INPUT),
            Ok((
                "",
                vec![
                    Blueprint::new(
                        1,
                        [
                            vec![(Material::Ore, 4)],
                            vec![(Material::Ore, 2)],
                            vec![(Material::Ore, 3), (Material::Clay, 14)],
                            vec![(Material::Ore, 2), (Material::Obsidian, 7)],
                        ]
                    ),
                    Blueprint::new(
                        2,
                        [
                            vec![(Material::Ore, 2)],
                            vec![(Material::Ore, 3)],
                            vec![(Material::Ore, 3), (Material::Clay, 8)],
                            vec![(Material::Ore, 3), (Material::Obsidian, 12)],
                        ]
                    )
                ]
            ))
        );
    }

    #[test]
    fn part1_test() {
        let (input, blueprints) = parse_blueprints(TEST_INPUT).unwrap();
        let blueprints_quality_levels_sum = blueprints.par_iter()
            .map(|blueprint|
                blueprint.id * max_geodes(blueprint, 24)
            )
            .sum::<usize>();

        assert_eq!(blueprints_quality_levels_sum, 33);
    }
}
