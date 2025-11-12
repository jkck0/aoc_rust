use itertools::Itertools;
use std::collections::HashMap;

pub struct Graph<'a> {
    indicies: HashMap<&'a str, usize>,
    distances: Vec<u32>,
    num_cities: usize,
}

impl<'a> Graph<'a> {
    fn new(num_cities: usize) -> Self {
        Self {
            indicies: HashMap::new(),
            distances: vec![u32::MAX; num_cities * num_cities],
            num_cities,
        }
    }

    fn add_connection(&mut self, from: &'a str, to: &'a str, dist: u32) {
        // add the cities to the indicies map if they aren't already
        let idx = self.indicies.len();
        self.indicies.entry(from).or_insert(idx);

        let idx = self.indicies.len();
        self.indicies.entry(to).or_insert(idx);

        let start = self.indicies[from];
        let end = self.indicies[to];
        self.distances[start * self.num_cities + end] = dist;
        self.distances[end * self.num_cities + start] = dist;
    }

    fn distances(&self, route: &[&usize]) -> Vec<u32> {
        let mut distances = vec![];

        for i in 0..route.len() - 1 {
            let start = route[i];
            let end = route[i + 1];
            distances.push(self.distances[start * self.num_cities + end])
        }

        distances
    }

    fn shortest_path(&self) -> Vec<u32> {
        let indicies: Vec<usize> = (0..self.num_cities).collect();

        indicies
            .iter()
            .permutations(self.num_cities)
            .map(|route| self.distances(&route))
            .min_by_key(|v| v.iter().sum::<u32>())
            .unwrap()
    }

    fn longest_path(&self) -> Vec<u32> {
        let indicies: Vec<usize> = (0..self.num_cities).collect();

        indicies
            .iter()
            .permutations(self.num_cities)
            .map(|route| self.distances(&route))
            .max_by_key(|v| v.iter().sum::<u32>())
            .unwrap()
    }
}

pub fn parse<'a>(data: &'a str) -> Graph<'a> {
    // Determine the number of cities from the number of lines in the file
    // The number of lines in the file (i.e. the number of edges in a complete graph with n nodes)
    // is equal to (n^2)/2 - n/2. A bit of algebra gets you the below formula to reverse this
    let num_cities = (1 + (((1 + 8 * data.lines().count()) as f64).sqrt() as usize)) / 2;
    let mut graph = Graph::new(num_cities);

    data.lines().for_each(|s| {
        let mut words = s.split_ascii_whitespace();
        let from = words.next().unwrap();
        let to = words.nth(1).unwrap();
        let dist = words.nth(1).unwrap().parse().unwrap();

        graph.add_connection(from, to, dist);
    });
    graph
}

pub fn part1(graph: &Graph) -> u32 {
    graph.shortest_path().iter().sum()
}

pub fn part2(graph: &Graph) -> u32 {
    graph.longest_path().iter().sum()
}
