use std::collections::{HashMap, hash_map::Entry};

use crate::util::disjoint_set::DisjointSetData;

#[derive(Debug, Clone, Copy)]
pub struct JunctionBox {
    x: i32,
    y: i32,
    z: i32,
}

pub fn parse(data: &str) -> Vec<JunctionBox> {
    data.lines()
        .map(|l| {
            let mut coords = l.split(',').map(|n| n.parse().unwrap());
            JunctionBox {
                x: coords.next().unwrap(),
                y: coords.next().unwrap(),
                z: coords.next().unwrap(),
            }
        })
        .collect()
}

// Stores junction boxes as indices into the vec.
#[derive(Debug, Clone, Copy)]
struct Connection {
    from: usize,
    to: usize,
}

fn get_distances(boxes: &[JunctionBox]) -> HashMap<u64, Connection> {
    // A map from squared distance to the pair of junction boxes with that distance
    let mut distances = HashMap::<u64, Connection>::new();
    // Find the squared distance between each pair of junction boxes
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            let (box_a, box_b) = (boxes[i], boxes[j]);
            let distance = ((box_a.x - box_b.x).abs() as u64).pow(2)
                + ((box_a.y - box_b.y).abs() as u64).pow(2)
                + ((box_a.z - box_b.z).abs() as u64).pow(2);

            // Add the connection to the map. If it was already there, oops, I need to change my implementation
            match distances.entry(distance) {
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(Connection { from: i, to: j });
                }
                Entry::Occupied(_) => {
                    panic!("multiple junction box pairs with the same distance");
                }
            }
        }
    }

    distances
}

pub fn part1(input: &[JunctionBox]) -> usize {
    let distances = get_distances(input);
    let mut distances_sorted: Vec<_> = distances.keys().collect();
    distances_sorted.sort_unstable();

    let mut circuits: DisjointSetData<_> = input.into_iter().collect();
    for &dist in distances_sorted.iter().take(1000) {
        let connection = distances.get(dist).unwrap();
        circuits.union(connection.from, connection.to);
    }

    let mut sets = circuits.sets();
    sets.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
    sets.into_iter().rev().take(3).map(|v| v.len()).product()
}

pub fn part2(input: &[JunctionBox]) -> u64 {
    let distances = get_distances(input);
    let mut distances_sorted: Vec<_> = distances.keys().collect();
    distances_sorted.sort_unstable();
    let mut distances_iter = distances_sorted.into_iter();

    let mut circuits: DisjointSetData<_> = input.into_iter().collect();
    loop {
        let dist = distances_iter.next().unwrap();
        let connection = distances.get(dist).unwrap();
        circuits.union(connection.from, connection.to);

        if circuits.num_sets() == 1 {
            break circuits[connection.from].x as u64 * circuits[connection.to].x as u64;
        }
    }
}
