use std::cmp::{max, min};

use crate::util::point::{Point, RIGHT};

pub fn parse(data: &str) -> Vec<Point> {
    data.lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &[Point]) -> u64 {
    let mut max_area = 0;
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let (a, b) = (input[i], input[j]);
            let area = ((a.x - b.x).abs() as u64 + 1) * ((a.y - b.y).abs() as u64 + 1);
            max_area = max(max_area, area);
        }
    }

    max_area
}

#[derive(Debug)]
struct Edge {
    start: Point,
    end: Point,
}

struct OrthographicPolygon {
    edges: Vec<Edge>,
}

impl OrthographicPolygon {
    fn from_vertices(vertices: &[Point]) -> Self {
        let mut edges = Vec::with_capacity(vertices.len());
        let mut max_x = 0;
        for i in 0..vertices.len() {
            max_x = max(max_x, vertices[i].x);

            edges.push(Edge {
                start: vertices[i],
                end: vertices[(i + 1) % vertices.len()],
            })
        }

        Self { edges }
    }

    fn contains_rect(&self, vertex_a: Point, vertex_b: Point) -> bool {
        // Since the vertices of the rectangle are always vertices of the polygon,
        // if the rectangle is a line, it will always be contained
        if vertex_a.x == vertex_b.x || vertex_a.y == vertex_b.y {
            return true;
        }

        let (max_x, max_y) = (max(vertex_a.x, vertex_b.x), max(vertex_a.y, vertex_b.y));
        let (min_x, min_y) = (min(vertex_a.x, vertex_b.x), min(vertex_a.y, vertex_b.y));

        // If the interior of the rectangle intersects any edges, then it is not contained
        for edge in self.edges.iter() {
            let is_left = max(edge.start.x, edge.end.x) <= min_x;
            let is_right = min(edge.start.x, edge.end.x) >= max_x;
            let is_top = max(edge.start.y, edge.end.y) <= min_y;
            let is_bottom = min(edge.start.y, edge.end.y) >= max_y;

            let intersecting = !(is_left || is_right || is_top || is_bottom);
            if intersecting {
                return false;
            }
        }

        true
    }
}

pub fn part2(input: &[Point]) -> u64 {
    let polygon = OrthographicPolygon::from_vertices(input);
    let mut max_area = 0;
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let (a, b) = (input[i], input[j]);

            let area = ((a.x - b.x).abs() as u64 + 1) * ((a.y - b.y).abs() as u64 + 1);
            if area > max_area && polygon.contains_rect(input[i], input[j]) {
                max_area = area;
            }
        }
    }

    max_area
}
