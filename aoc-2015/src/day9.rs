use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

const PROBLEM: &str = include_str!("input/day9.txt");

#[derive(Debug)]
struct Edge<'a> {
    dest: &'a str,
    dist: usize,
}

fn build_graph(problem: &str) -> HashMap<&str, Vec<Edge>> {
    let mut graph = HashMap::<&str, Vec<Edge>>::new();
    for line in problem.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        if let Some(city) = graph.get_mut(split[0]) {
            city.push(Edge {
                dest: split[2],
                dist: str::parse::<usize>(split[4]).unwrap(),
            });
        } else {
            graph.insert(
                split[0],
                vec![Edge {
                    dest: split[2],
                    dist: str::parse::<usize>(split[4]).unwrap(),
                }],
            );
        }

        if let Some(city) = graph.get_mut(split[2]) {
            city.push(Edge {
                dest: split[0],
                dist: str::parse::<usize>(split[4]).unwrap(),
            });
        } else {
            graph.insert(
                split[2],
                vec![Edge {
                    dest: split[0],
                    dist: str::parse::<usize>(split[4]).unwrap(),
                }],
            );
        }
    }

    graph
}

fn backtrack<'a>(
    city: &'a str,
    dist: usize,
    graph: &HashMap<&'a str, Vec<Edge<'a>>>,
    visited: &mut HashSet<&'a str>,
    path_lengths: &mut Vec<usize>,
) {
    if visited.len() == graph.keys().len() {
        path_lengths.push(dist);
    } else {
        for edge in graph.get(city).unwrap().iter() {
            if !visited.contains(edge.dest) {
                visited.insert(edge.dest);
                backtrack(edge.dest, dist + edge.dist, graph, visited, path_lengths);
                visited.remove(edge.dest);
            }
        }
    }
}

fn find_shortest_round_trip(graph: &HashMap<&str, Vec<Edge>>) -> Option<usize> {
    let mut min: Option<usize> = None;

    for &start in graph.keys() {
        let mut visited = HashSet::<&str>::new();
        visited.insert(start);
        let mut path_lengths = vec![];

        backtrack(start, 0, &graph, &mut visited, &mut path_lengths);
        match (min, path_lengths.iter().min()) {
            (Some(m1), Some(m2)) => {
                if *m2 < m1 {
                    min = Some(*m2);
                }
            }
            (None, Some(m2)) => {
                min = Some(*m2);
            }
            _ => {}
        }
    }

    min
}

fn find_longest_round_trip(graph: &HashMap<&str, Vec<Edge>>) -> Option<usize> {
    let mut max: Option<usize> = None;

    for &start in graph.keys() {
        let mut visited = HashSet::<&str>::new();
        visited.insert(start);
        let mut path_lengths = vec![];

        backtrack(start, 0, &graph, &mut visited, &mut path_lengths);
        match (max, path_lengths.iter().max()) {
            (Some(m1), Some(m2)) => {
                if *m2 > m1 {
                    max = Some(*m2);
                }
            }
            (None, Some(m2)) => {
                max = Some(*m2);
            }
            _ => {}
        }
    }

    max
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let graph = build_graph(PROBLEM);
    let min = find_shortest_round_trip(&graph);

    println!("{}", min.unwrap());
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let graph = build_graph(PROBLEM);
    let min = find_longest_round_trip(&graph);

    println!("{}", min.unwrap());
    Ok(())
}
