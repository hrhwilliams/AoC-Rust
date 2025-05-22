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

fn find_shorted_round_trip(start: &str, graph: &HashMap<&str, Vec<Edge>>) -> usize {
    let mut visited = HashSet::<&str>::new();
    visited.insert(start);
    let mut path_lengths= vec!();
    let mut path = vec!();

    backtrack(start, 0, &graph, &mut visited, &mut path_lengths, &mut path);

    *path_lengths.iter().min().unwrap()
}

fn backtrack<'a>(
    city: &'a str,
    dist: usize,
    graph: &HashMap<&'a str, Vec<Edge<'a>>>,
    visited: &mut HashSet<&'a str>,
    path_lengths: &mut Vec<usize>,
    path: &mut Vec<&'a str>
) {
    path.push(city);

    if visited.len() == graph.keys().len() {
        println!("{:?}", path);
        path_lengths.push(dist);
    } else {
        for edge in graph.get(city).unwrap().iter() {
            if !visited.contains(edge.dest) {
                let mut new_path = path.clone();
                visited.insert(edge.dest);
                backtrack(edge.dest, dist + edge.dist, graph, visited, path_lengths, &mut new_path);
                visited.remove(edge.dest);
            }
        }
    }
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let graph = build_graph(PROBLEM);
    let path_length = find_shorted_round_trip("Tristram", &graph);

    println!("min = {}", path_length);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
