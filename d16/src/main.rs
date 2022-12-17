use pathfinding::directed::bfs::bfs;
use pqueue::Queue;
use scanf::sscanf;
use std::{
    cmp::{max, min, Ordering},
    collections::{hash_map::Entry, HashMap, HashSet},
    time::Instant,
};

// const INPUT: &str = TEST;
const INPUT: &str = REAL;
// 1022 too low
// 1138 too low
// 1611 too low
// 1747
// okay I'm giving up now

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, (usize, Vec<String>)>,
    sorted_nodes: Vec<String>,
}

// okay we can dedup by looking at on_nodes, current position

// this one cool dynamic programming trick they don't want you to know!
// simply cache the whole algorithm!!!
#[derive(Debug, Clone, PartialEq, Eq)]
struct Path {
    // it's the current increment projected out to turn 30 plus the action_score
    // a better heuristic might be to look at nearby off nodes too
    heuristic: isize,
    // an empty string means turn the valve on, why not
    actions: Vec<String>,
    // scoring the actions but not any waiting around
    action_score: usize,
    action_score_increment: usize,
    current_node: String,
    on_nodes: HashSet<String>,
    sorted_off_nonzero_nodes: Vec<String>,
    visited_since_last_turnon: HashSet<String>,
    // we're just going to hard-dedup
    previous_positions: HashMap<(Vec<String>, String), usize>,
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score()
            .cmp(&other.score())
            // bigger is better
            .reverse()
    }
}

impl PartialOrd for Path {
    // we should do something on a tie maybe
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Path {
    fn new(graph: &Graph) -> Self {
        Path {
            heuristic: 0,
            actions: vec![],
            action_score: 0,
            action_score_increment: 0,
            current_node: "AA".to_string(),
            on_nodes: HashSet::new(),
            sorted_off_nonzero_nodes: graph.sorted_nodes.clone(),
            visited_since_last_turnon: HashSet::new(),
            previous_positions: HashMap::new(),
        }
    }
    // if none, then it was a stupid move
    fn update(&self, graph: &Graph, action: String) -> Option<Self> {
        let mut new_self = self.clone();
        if action == "" {
            if graph.nodes[&self.current_node].0 > 0
                && new_self.on_nodes.insert(self.current_node.clone())
            {
                new_self.action_score_increment += graph.nodes[&self.current_node].0;
                // let's keep front of the list of off nodes updated too
                while new_self
                    .sorted_off_nonzero_nodes
                    .last()
                    .as_ref()
                    .map(|last| new_self.on_nodes.contains(*last))
                    .unwrap_or(false)
                {
                    new_self.sorted_off_nonzero_nodes.pop();
                }

                new_self.visited_since_last_turnon = HashSet::new();
            } else {
                // it was a stupid move i.e. turning on a zero cost node, or
                // turning on one that was already on so fuck off
                return None;
            }
        } else {
            // do we have a pointless loop?
            if !new_self.visited_since_last_turnon.insert(action.clone()) {
                return None;
            }
            new_self.current_node = action.clone();
        }

        new_self.actions.push(action);
        new_self.action_score += self.action_score_increment; // old increment
        new_self.heuristic = new_self.score() as isize      //  * 100
         // - cumulative_weighted_cost_to_visit_off_nodes(graph, &new_self.on_nodes, new_self.current_node.clone()) as isize
                                                        // * (30-new_self.actions.len()as isize)
                                                        // * 10000
                                                        // + new_self.on_nodes.len()as isize
        - new_self
            .sorted_off_nonzero_nodes
            .last()
            .as_ref()
            .map(|last| {
                shortest_path_to_node(graph, new_self.current_node.clone(), (*last).clone()).len() as isize
                * graph.nodes[last.clone()].0 as isize
            })
            .unwrap_or(-1); // if there's no nodes left to visit, this is a good path
                            // * (30-new_self.actions.len()as isize)

        //     *10
        // -new_self.actions.len() as isize;        // longer for no reason is bad
        // (Vec<String>, String)
        let mut x = new_self.on_nodes.iter().cloned().collect::<Vec<_>>();
        x.sort();
        let position = (x, new_self.current_node.clone());
        match new_self.previous_positions.entry(position) {
            Entry::Occupied(mut old_entry) => {
                if *old_entry.get() < new_self.action_score {
                    old_entry.insert(new_self.action_score);
                } else {
                    return None; // it's worse than the old way
                }
            }
            Entry::Vacant(x) => {
                x.insert(new_self.action_score);
            }
        }

        Some(new_self)
    }

    fn all_valves_on(&self, graph: &Graph) -> bool {
        // self.on_nodes
        //     .difference(&HashSet::from_iter(graph.sorted_nodes.iter().cloned()))
        //     .next()
        //     .is_none()
        self.sorted_off_nonzero_nodes.is_empty()
    }

    fn score(&self) -> usize {
        let remaining_turns = 30 - self.actions.len();
        self.action_score + remaining_turns * self.action_score_increment
    }
}

// we can do way better than this come on
fn cumulative_weighted_cost_to_visit_off_nodes(
    graph: &Graph,
    on_nodes: &HashSet<String>,
    current_node: String,
) -> usize {
    let mut score = 0;
    for (k, v) in &graph.nodes {
        if v.0 != 0 && !on_nodes.contains(k) {
            score += v.0 * shortest_path_to_node(graph, current_node.clone(), k.clone()).len();
        }
    }
    score
}

fn shortest_path_to_node(graph: &Graph, current_node: String, target_node: String) -> Vec<String> {
    bfs(
        &current_node,
        |node: &String| graph.nodes[node].1.iter().cloned(),
        |node: &String| *node == target_node,
    )
    .unwrap()
}

fn main() {
    let graph = parse();
    let dummy_path = Path::new(&graph);
    let mut paths = Queue::new();
    paths.push(dummy_path);
    let mut finished_path = Path::new(&graph);

    let algo_start = Instant::now();

    while let Some(top) = paths.pop() {
        // currently we just keep searching until we've hit every path with length 30
        // but we can finish early if we turned all valves on because there's nothing left to do
        if top.actions.len() == 30 || top.all_valves_on(&graph) {
            let top_score = top.score();
            finished_path = min(finished_path, top);
            if top_score > 1747 {
                break;
            }
        } else {
            // get neighbours, push to pqueue
            for tunnel in &graph.nodes[&top.current_node].1 {
                if let Some(new_candidate) = top.update(&graph, tunnel.clone()) {
                    paths.push(new_candidate);
                }
            }
            if let Some(new_candidate) = top.update(&graph, "".to_string()) {
                paths.push(new_candidate);
            }
        }
    }
    // let mut max_score = 0;
    // for path in &finished_paths {
    //     max_score = max(max_score, path.score());
    //     println!("{}", path.score());
    // }
    let done = Instant::now();
    println!("{}: {:?}", finished_path.score(), finished_path);
    println!("{:?}", done - algo_start);
    // println!("{}", max_score);

    // let test_actions = vec![
    //     "DD".to_string(),
    //     "".to_string(),
    //     "CC".to_string(),
    //     "BB".to_string(),
    //     "".to_string(),
    //     "AA".to_string(),
    //     "II".to_string(),
    //     "JJ".to_string(),
    //     "".to_string(),
    //     "II".to_string(),
    //     "AA".to_string(),
    //     "DD".to_string(),
    //     "EE".to_string(),
    //     "FF".to_string(),
    //     "GG".to_string(),
    //     "HH".to_string(),
    //     "".to_string(),
    //     "GG".to_string(),
    //     "FF".to_string(),
    //     "EE".to_string(),
    //     "".to_string(),
    //     "DD".to_string(),
    //     "CC".to_string(),
    //     "".to_string(),
    // ];

    // let mut test_path = Path::new(&graph);
    // for test_move in test_actions {
    //     test_path = test_path.update(&graph, test_move).unwrap();
    // }
    // let score = test_path.score();
    // assert_eq!(score, 1651);
}

// urgh
fn parse() -> Graph {
    let mut nodes = HashMap::new();
    let mut non_zero_nodes = Vec::new();

    for line in INPUT.lines() {
        let mut id = String::new();
        let mut flow = 0;
        let mut targets_str = String::new();
        sscanf!(line, "Valve {} has flow rate={}; {}", id, flow, targets_str).unwrap();
        targets_str = targets_str.strip_prefix("tunnel").unwrap().to_string();
        if let Some(s) = targets_str.strip_prefix("s") {
            targets_str = s.to_string();
        }
        targets_str = targets_str.strip_prefix(" lead").unwrap().to_string();

        if let Some(s) = targets_str.strip_prefix("s") {
            targets_str = s.to_string();
        }
        targets_str = targets_str.strip_prefix(" to valve").unwrap().to_string();
        if let Some(s) = targets_str.strip_prefix("s") {
            targets_str = s.to_string();
        }
        let mut targets = vec![];
        for target in targets_str.split(',') {
            targets.push(target.trim().to_string());
        }
        if flow > 0 {
            non_zero_nodes.push(id.clone());
        }
        nodes.insert(id, (flow, targets));
    }
    non_zero_nodes.sort();
    Graph {
        nodes,
        sorted_nodes: non_zero_nodes,
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
