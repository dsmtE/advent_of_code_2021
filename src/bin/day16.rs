use std::collections::{HashSet, HashMap};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, line_ending},
    multi::{separated_list0, separated_list1},
    sequence::preceded,
    IResult,
};

use petgraph::{
    dot::{Config, Dot},
    Graph, stable_graph::NodeIndex,
};

const INPUT: &str = advent_of_code::get_input!();

use advent_of_code::iterator_to_string;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow_rate: u32,
}

struct ValveInfos {
    valve: Valve,
    neighbours: HashSet<String>,
}

impl std::fmt::Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Valve {} has flow rate={}", self.name, self.flow_rate)
    }
}

impl std::fmt::Display for ValveInfos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let one_neighbour = self.neighbours.len() == 1;
        write!(
            f,
            "{}: tunnel{} lead{} to valve{} {}",
            self.valve,
            if one_neighbour { "" } else { "s" },
            if one_neighbour { "s" } else { "" },
            if one_neighbour { "" } else { "s" },
            iterator_to_string(self.neighbours.iter(), ", "),
        )
    }
}

fn parse_line(input: &str) -> IResult<&str, ValveInfos> {
    let (input, name) = preceded(tag("Valve "), take(2u8))(input)?;
    let (input, flow_rate) = preceded(tag(" has flow rate="), complete::u32)(input)?;
    let (input, tunnels) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list0(tag(", "), take(2u8)),
    )(input)?;

    Ok((
        input,
        ValveInfos {
            valve: Valve {
                name: name.to_owned(),
                flow_rate,
            },
            neighbours: tunnels.iter().map(|&s| s.to_owned()).collect(),
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<ValveInfos>> {
    separated_list1(line_ending, parse_line)(input)
}

fn build_valves_graph(valve_infos: &[ValveInfos]) -> (Graph<Valve, ()>, HashMap<String, NodeIndex>) {
    let mut graph = Graph::new();
    let mut valve_nodes_indices = std::collections::HashMap::new();
    for valve_info in valve_infos {
        let node_index = graph.add_node(valve_info.valve.clone());
        valve_nodes_indices.insert(valve_info.valve.name.clone(), node_index);
    }
    for valve_info in valve_infos {
        let from = valve_nodes_indices[&valve_info.valve.name];
        for to_name in &valve_info.neighbours {
            let to = valve_nodes_indices[to_name];
            graph.add_edge(from, to, ());
        }
    }
    (graph, valve_nodes_indices)
}

fn relieved_for(path: &[Valve]) -> u32 {
    todo!()
}
fn main() {
    // let (_, sensor_infos) = parse(INPUT).unwrap();
}

#[cfg(test)]
mod tests {
    use petgraph::algo::dijkstra;

    use super::*;
    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn parsing_test() {
        let (_, valve_infos) = parse(TEST_INPUT).unwrap();
        println!("{}", iterator_to_string(&valve_infos, "\n"));
    }

    #[test]
    fn simple_case() {
        let (_, valve_infos) = parse(TEST_INPUT).unwrap();
        let (graph, valve_nodes_indices) = build_valves_graph(&valve_infos);

        // Node index to valve name 
        let valve_names: HashMap<NodeIndex, String> = valve_nodes_indices.iter()
            .map(|(name, &node_index)| (node_index, name.clone()))
            .collect();

        let distances: HashMap<NodeIndex, HashMap<NodeIndex, usize>> = valve_infos.iter()
            .map(|ValveInfos { valve: Valve{ name, .. }, .. }| {
                let start = valve_nodes_indices[name];
                (start, dijkstra(&graph, start, None, |_| 1))
            })
            .collect();
        
        // valve that can be opened have a flow rate > 0
        let valves_to_open: Vec<&Valve> =  valve_infos.iter()
            .map(|ValveInfos { valve, .. }| valve)
            .filter(|Valve { flow_rate, .. }| *flow_rate > 0)
            .collect();
        
        let paths_in_minutes_from_AA = dijkstra(&graph, valve_nodes_indices["AA"], None, |_| 1);
        println!("{:?}", paths_in_minutes_from_AA.iter().map(|(node_index, min)| (&valve_names[&node_index], min)).collect::<Vec<_>>());

    }
}
