
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::cmp::Ordering;

use num_traits::Zero;

pub fn dijkstra<Node, Cost, FN, IN, FS>(
    starts: impl IntoIterator<Item = Node>,
    mut successors: FN,
    mut success: FS,
) -> Option<(Vec<Node>, Cost)>
where
    Node: Eq + Hash + Clone,
    Cost: Zero + Ord + Copy,
    FN: FnMut(&Node) -> IN,
    IN: IntoIterator<Item = (Node, Cost)>,
    FS: FnMut(&Node) -> bool,
{
    let (parents, reached) = run_dijkstra(starts, &mut successors, &mut success);
    reached.map(|target| {
        (
            reverse_path(&parents, |(p, _)| p.clone(), target.clone()),
            parents[&target].1,
        )
    })
}

fn run_dijkstra<Node, Cost, FN, IN, FS>(
    starts: impl IntoIterator<Item = Node>,
    successors: &mut FN,
    stop: &mut FS,
) -> (HashMap<Node, (Option<Node>, Cost)>, Option<Node>)
where
    Node: Eq + Hash + Clone,
    Cost: Zero + Ord + Copy,
    FN: FnMut(&Node) -> IN,
    IN: IntoIterator<Item = (Node, Cost)>,
    FS: FnMut(&Node) -> bool,
{
    let mut to_visit = BinaryHeap::new();
    let mut parent: HashMap<Node, (Option<Node>, Cost)> = HashMap::default();
    
    for start in starts {
        to_visit.push(SmallestHolder {
            cost: Zero::zero(),
            node: start.clone(),
        });
        parent.insert(start.clone(), (None, Zero::zero()));
    }

    let mut target_reached = None;

    while let Some(SmallestHolder { cost, node }) = to_visit.pop() {
        if stop(&node) {
            target_reached = Some(node);
            break;
        }

        for (neighbor, move_cost) in successors(&node) {
            let new_cost = cost + move_cost;

            let is_empty_or_less_costly = parent
                    .get(&neighbor)
                    .map_or(true, |&(_, current_cost)| new_cost < current_cost);

            // If we already have a cheaper path to `node`, do nothing.
            if is_empty_or_less_costly {
                parent.insert(neighbor.clone(), (Some(node.clone()), new_cost));
                to_visit.push(SmallestHolder {
                    cost: new_cost,
                    node: neighbor,
                });
            }
        }
    }
    (parent, target_reached)
}

fn reverse_path<Node, V, F>(parents: &HashMap<Node, V>, mut parent: F, start: Node) -> Vec<Node>
where
    Node: Eq + Hash + Clone,
    F: FnMut(&V) -> Option<Node>,
{
    let mut current = Some(start);
    std::iter::from_fn(|| {
        current.clone().and_then(|c| {
            parents.get(&c).map(|p| {
                current = parent(p);
                c
            })
        })
    })
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
    .collect()
}

struct SmallestHolder<Cost, Node> {
    cost: Cost,
    node: Node,
}

impl<Cost: PartialEq, Node> PartialEq for SmallestHolder<Cost, Node> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<Cost: PartialEq, Node> Eq for SmallestHolder<Cost, Node> {}

impl<Cost: Ord, Node> PartialOrd for SmallestHolder<Cost, Node> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Cost: Ord, Node> Ord for SmallestHolder<Cost, Node> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut graph = HashMap::new();

        graph.insert('A', vec![('B', 3), ('C', 2)]);
        graph.insert('B', vec![('C', 5), ('D', 2), ('G', 7)]);
        graph.insert('C', vec![('E', 2), ('F', 9)]);
        graph.insert('D', vec![('E', 8), ('F', 1)]);
        graph.insert('E', vec![('G', 3)]);
        graph.insert('F', vec![('G', 6), ('H', 7), ('K', 8)]);
        graph.insert('G', vec![('I', 6), ('J', 9)]);
        graph.insert('H', vec![('I', 7), ('J', 2)]);
        graph.insert('I', vec![('K', 4)]);
        graph.insert('J', vec![('K', 6), ('L', 4)]);
        graph.insert('K', vec![('L', 5)]);
        graph.insert('L', vec![]);

        let (path, cost) = dijkstra(&'A', |node| graph.get(node).unwrap().clone(), |node| *node == 'L').unwrap();
        assert_eq!(path, vec!['A', 'B', 'D', 'F', 'K', 'L']);
        assert_eq!(cost, 19);

        let (path, cost) = dijkstra(&'C', |node| graph.get(node).unwrap().clone(), |node| *node == 'J').unwrap();
        assert_eq!(path, vec!['C', 'E', 'G', 'J']);
        assert_eq!(cost, 14);
    }
}