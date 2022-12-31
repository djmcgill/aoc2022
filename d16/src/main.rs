use fxhash::FxHashMap as HashMap;
use pathfinding::directed::dijkstra::dijkstra_all;
use pqueue::Queue;
use scanf::sscanf;
use std::{
    cmp::{max, min, Ordering},
    collections::hash_map::Entry,
    time::Instant,
};

// const INPUT: &str = TEST;
const INPUT: &str = REAL;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
struct NodeId(usize);

#[derive(Clone, Debug)]
struct Node {
    weight: usize,
    connections: Vec<NodeId>,
}

#[derive(Debug, Clone)]
struct Graph {
    start: NodeId,
    nodes: HashMap<NodeId, Node>,
    routes_with_dist: HashMap<(NodeId, NodeId), usize>,
    non_zero_nodes: u64,
}

fn main() {
    let start_time = Instant::now();
    let graph_p1 = parse(INPUT);
    let algo_start = Instant::now();
    let p1 = p1(&graph_p1);
    let p1_done = Instant::now();

    let graph_p2 = parse(INPUT);
    let p2 = p2(&graph_p2);
    let p2_done = Instant::now();

    println!("p1: {}", p1);
    println!("p2: {}", p2);
    println!("parse: {:?}", algo_start - start_time);
    println!("p1: {:?}", p1_done - start_time);
    println!("p2: {:?}", p2_done - p1_done);

    // parse: 531.9µs
    // p1: 27.6257ms
    // p2: 505.5361ms
}

fn p1(graph: &Graph) -> usize {
    let mut dedup_lookup: HashMap<(NodeId, u64), usize> = HashMap::default();

    let dummy_path = P1Path::new(graph);
    let mut paths = Queue::new();
    paths.push(dummy_path);
    let mut finished_path = P1Path::new(graph);

    while let Some(top) = paths.pop() {
        if top < finished_path {
            finished_path = top.clone();
        }

        if top.path.time_so_far >= 30 || top.all_valves_on(graph) {
            // we're done, this path can have no more children
        } else {
            // get the next candidates
            for non_zero_node in remaining_nodes(top.remaining_nodes) {
                if let Some(candidate) = top.update(graph, non_zero_node) {
                    if update_dedup_cache_p1(&mut dedup_lookup, &candidate) {
                        paths.push(candidate);
                    }
                }
            }
        }
    }
    finished_path.score
}

fn p2(graph: &Graph) -> usize {
    // if we're in the same place, and have visited the same nodes, ignore worse
    let mut dedup_lookup: HashMap<(NodeId, NodeId, u64), usize> = HashMap::default();

    let dummy_path = P2Path::new(graph);
    let mut paths = Queue::new();
    paths.push(dummy_path);
    let mut finished_path = P2Path::new(graph);

    while let Some(top) = paths.pop() {
        if top < finished_path {
            finished_path = top.clone();
        }

        let mut candidates = vec![];

        if top.all_valves_on(graph) {
            // we're done, this path can have no more children
        } else {
            for non_zero_node in remaining_nodes(top.remaining_nodes) {
                let (your_node, elephant_node) = (
                    top.path.you.valves.last().unwrap_or(&graph.start),
                    top.path.elephant.valves.last().unwrap_or(&graph.start),
                );
                // there's no point adding a given node to the further away path
                let your_travel_time = graph.routes_with_dist[&(*your_node, non_zero_node)] + 1;
                let elephant_travel_time =
                    graph.routes_with_dist[&(*elephant_node, non_zero_node)] + 1;
                let (is_you, travel_time) = if your_travel_time < elephant_travel_time {
                    (true, your_travel_time)
                } else {
                    (false, elephant_travel_time)
                };

                if let Some(candidate) = top.update(graph, non_zero_node, is_you, travel_time) {
                    candidates.push(candidate);
                }
            }
        }

        for candidate in candidates {
            if update_dedup_cache_p2(graph, &mut dedup_lookup, &candidate, finished_path.score) {
                paths.push(candidate);
            }
        }
    }
    finished_path.score
}

type P1Path = Path<P1RawPath>;
type P2Path = Path<P2RawPath>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct P1RawPath {
    valves: Vec<NodeId>,
    time_so_far: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct P2RawPath {
    you: P1RawPath,
    elephant: P1RawPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Path<P> {
    path: P,

    // cache
    score: usize,
    score_increment_so_far: usize,
    score_so_far: usize,
    remaining_nodes: u64,
}
impl<P: Eq> Ord for Path<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        // backwards for min pqueue
        other.score.cmp(&self.score)
    }
}

impl<P: Eq> PartialOrd for Path<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<P: Default> Path<P> {
    fn new(graph: &Graph) -> Self {
        Self {
            path: P::default(),
            score: 0,
            score_increment_so_far: 0,
            score_so_far: 0,
            remaining_nodes: graph.non_zero_nodes,
        }
    }
    fn all_valves_on(&self, _graph: &Graph) -> bool {
        self.remaining_nodes == 0
    }
}

impl P1Path {
    fn update(&self, graph: &Graph, dest: NodeId) -> Option<Self> {
        let current_node = self.path.valves.last().unwrap_or(&graph.start);
        let travel_time = graph.routes_with_dist[&(*current_node, dest)];
        let dt = travel_time + 1; // got to turn valve on

        if self.path.time_so_far + dt <= 30 {
            let mut new_self = self.clone();
            new_self.path.valves.push(dest);
            new_self.remaining_nodes ^= 1 << dest.0;

            new_self.score_so_far += dt * self.score_increment_so_far;
            new_self.path.time_so_far += dt;
            new_self.score_increment_so_far += graph.nodes[&dest].weight;

            let new_score = new_self.score(graph);
            new_self.score = new_score;
            Some(new_self)
        } else {
            None
        }
    }

    fn score(&self, _graph: &Graph) -> usize {
        let remaining_time = 30 - self.path.time_so_far;
        self.score_so_far + remaining_time * self.score_increment_so_far
    }
}

impl P2Path {
    fn update(
        &self,
        graph: &Graph,
        dest: NodeId,
        your_move: bool,
        travel_dt: usize,
    ) -> Option<Self> {
        // how long will it take us to get there
        let (time_so_far, other_time_so_far) = if your_move {
            (self.path.you.time_so_far, self.path.elephant.time_so_far)
        } else {
            (self.path.elephant.time_so_far, self.path.you.time_so_far)
        };

        let new_time = max(time_so_far + travel_dt, other_time_so_far);
        let old_time = max(time_so_far, other_time_so_far);
        let dt = new_time - old_time;
        if time_so_far + travel_dt <= 26 {
            // okay now update everything
            let mut new_self = self.clone();
            if your_move {
                new_self.path.you.valves.push(dest);
                new_self.path.you.time_so_far += travel_dt;
            } else {
                new_self.path.elephant.valves.push(dest);
                new_self.path.elephant.time_so_far += travel_dt;
            }
            new_self.remaining_nodes ^= 1 << dest.0;
            new_self.score_so_far += dt * self.score_increment_so_far;
            new_self.score_increment_so_far += graph.nodes[&dest].weight;

            let new_score = new_self.score(graph);
            new_self.score = new_score;

            Some(new_self)
        } else {
            None
        }
    }

    fn score(&self, _graph: &Graph) -> usize {
        let remaining_time = 26 - max(self.path.you.time_so_far, self.path.elephant.time_so_far);
        self.score_so_far + remaining_time * self.score_increment_so_far
    }
}

fn update_dedup_cache_p1(
    dedup_lookup: &mut HashMap<(NodeId, u64), usize>,
    candidate: &P1Path,
) -> bool {
    if let Some(&your_node) = candidate.path.valves.last() {
        match dedup_lookup.entry((your_node, candidate.remaining_nodes)) {
            Entry::Vacant(entry) => {
                entry.insert(candidate.score);
                true
            }
            Entry::Occupied(mut entry) => {
                let old_score = entry.get_mut();
                if *old_score < candidate.score {
                    *old_score = candidate.score;
                    true
                } else {
                    false
                }
            }
        }
    } else {
        false
    }
}

fn update_dedup_cache_p2(
    graph: &Graph,
    dedup_lookup: &mut HashMap<(NodeId, NodeId, u64), usize>,
    candidate: &P2Path,
    max_score: usize,
) -> bool {
    if let (Some(&your_node), Some(&elephant_node)) = (
        candidate.path.you.valves.last(),
        candidate.path.elephant.valves.last(),
    ) {
        let score_upper_bound = upper_bound(candidate, graph);
        if score_upper_bound >= max_score {
            match dedup_lookup.entry((your_node, elephant_node, candidate.remaining_nodes)) {
                Entry::Vacant(entry) => {
                    entry.insert(candidate.score);
                    true
                }
                Entry::Occupied(mut entry) => {
                    let old_score = entry.get_mut();
                    if *old_score < candidate.score {
                        *old_score = candidate.score;
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    } else {
        true
    }
}

fn upper_bound(candidate: &P2Path, graph: &Graph) -> usize {
    // what's the best possible score this node can get? that's if
    // we go to the nearest node, then all the rest are immediately after that one
    // from largest to smallest
    let mut extra_score = 0;
    let mut extra_increment = 0;

    let mut remaining_time = 26
        - min(
            candidate.path.elephant.time_so_far,
            candidate.path.you.time_so_far,
        );

    let mut node_dists = vec![];
    let mut remaining_candidates = vec![];
    for node_id in remaining_nodes(candidate.remaining_nodes) {
        let weight = graph.nodes[&node_id].weight;

        if let (Some(&your_node), Some(&elephant_node)) = (
            candidate.path.you.valves.last(),
            candidate.path.elephant.valves.last(),
        ) {
            let your_travel_time = graph.routes_with_dist[&(your_node, node_id)] + 1;
            let elephant_travel_time = graph.routes_with_dist[&(elephant_node, node_id)] + 1;
            node_dists.push((
                min(your_travel_time, elephant_travel_time),
                -(weight as isize),
            ));
        }

        remaining_candidates.push(weight);
    }

    node_dists.sort();
    remaining_candidates.sort();

    if !node_dists.is_empty() && remaining_time >= node_dists[0].0 {
        remaining_time -= node_dists[0].0;
        extra_increment += (-node_dists[0].1) as usize;
    }

    while remaining_time > 1 {
        if let Some(biggest_weight) = remaining_candidates.pop() {
            extra_score += 2 * extra_increment;
            extra_increment += biggest_weight;
            remaining_time -= 2;
        } else {
            break;
        }
    }

    candidate.score + extra_score + remaining_time * extra_increment
}

fn remaining_nodes(remaining_nodes: u64) -> impl Iterator<Item = NodeId> {
    (0..60)
        .filter(move |i| 1 << i & remaining_nodes != 0)
        .map(NodeId)
}

// urgh
fn parse(input: &str) -> Graph {
    let mut nodes = HashMap::default();
    let mut non_zero_nodes = 0u64;
    let mut start = None;
    let mut name_lookup = HashMap::default();

    // this is a huge hack lol
    for (id, line) in input.lines().enumerate() {
        let mut id_str = String::new();
        let mut whatever = String::new();
        sscanf!(line, "Valve {} has {}", id_str, whatever,).unwrap();
        name_lookup.insert(id_str, NodeId(id));
    }

    for (id, line) in input.lines().enumerate() {
        let mut id_str = String::new();
        let mut flow = 0;
        let mut targets_str = String::new();
        sscanf!(
            line,
            "Valve {} has flow rate={}; {}",
            id_str,
            flow,
            targets_str
        )
        .unwrap();
        let id = NodeId(id);

        if id_str == "AA" {
            start = Some(id);
        }
        targets_str = targets_str.strip_prefix("tunnel").unwrap().to_string();
        if let Some(s) = targets_str.strip_prefix('s') {
            targets_str = s.to_string();
        }
        targets_str = targets_str.strip_prefix(" lead").unwrap().to_string();

        if let Some(s) = targets_str.strip_prefix('s') {
            targets_str = s.to_string();
        }
        targets_str = targets_str.strip_prefix(" to valve").unwrap().to_string();
        if let Some(s) = targets_str.strip_prefix('s') {
            targets_str = s.to_string();
        }
        let mut connections = vec![];
        for target_str in targets_str.split(',') {
            connections.push(name_lookup[target_str.trim()]);
        }
        if flow > 0 {
            non_zero_nodes |= 1 << id.0;
        }
        nodes.insert(
            id,
            Node {
                weight: flow, // usize,
                connections,  // Vec<usize>,
                              // name: id_str, // String
            },
        );
    }

    // fixme: we should take advantage of this being bidirectional
    // fixme: floyd-warshall?
    let mut routes_with_dist = HashMap::default();

    for node_id_start in remaining_nodes(non_zero_nodes) {
        let all_reachable = dijkstra_all(&node_id_start, |node: &NodeId| {
            nodes[node].connections.iter().map(|x| (*x, 1))
        });
        for (node_id_end, (_, cost)) in all_reachable {
            routes_with_dist.insert((node_id_start, node_id_end), cost);
        }
    }

    let all_reachable = dijkstra_all(start.as_ref().unwrap(), |node: &NodeId| {
        nodes[node].connections.iter().map(|x| (*x, 1))
    });
    for (node_id_end, (_, cost)) in all_reachable {
        routes_with_dist.insert((start.unwrap(), node_id_end), cost);
    }

    Graph {
        start: start.unwrap(),
        nodes,
        routes_with_dist,
        // name_lookup,
        non_zero_nodes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let graph = parse(TEST);

        let test_actions = vec![
            "DD".to_string(),
            "BB".to_string(),
            "JJ".to_string(),
            "HH".to_string(),
            "EE".to_string(),
            "CC".to_string(),
        ];

        let mut test_path = Path::new(&graph);
        for test_move in test_actions {
            test_path = test_path
                .update(&graph, graph.name_lookup[&test_move])
                .unwrap();
        }
        let score = test_path.score(&graph);
        assert_eq!(score, 1651);
        assert_eq!(test_path.score, 1651);
    }
}

const REAL: &str = include_str!("real.txt");
const TEST: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
