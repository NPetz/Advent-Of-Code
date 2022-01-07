use std::{
    cmp::{max, min},
    collections::HashMap,
    str::FromStr,
};

use crate::utils::read_input;

pub fn run() {
    println!("\nDay 5");

    let contents = read_input("five");

    println!("\nPart I");

    part_one(&contents);

    println!("\nPart II");

    part_two(&contents);
}

fn part_one(input: &str) {
    let points: Vec<Point> = input
        .lines()
        .map(|l| Line::from_str(l).unwrap().get_distinct_hortogonal_points())
        .flatten()
        .collect();

    let counts = points.into_iter().fold(HashMap::new(), |mut map, p| {
        map.entry(p).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let counts_more_than_one = counts.into_values().filter(|&v| v > 1).count();

    println!("{}", counts_more_than_one)
}

fn part_two(input: &str) {
    let points: Vec<Point> = input
        .lines()
        .map(|l| Line::from_str(l).unwrap().get_distinct_points())
        .flatten()
        .collect();

    let counts = points.into_iter().fold(HashMap::new(), |mut map, p| {
        map.entry(p).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let counts_more_than_one = counts.into_values().filter(|&v| v > 1).count();

    println!("{}", counts_more_than_one)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();

        match coords[..] {
            [x, y] => Ok(Point { x, y }),
            _ => Err("error parsing point".to_string()),
        }
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("->").map(|s| s.trim()).collect::<Vec<&str>>()[..] {
            [s, e] => Ok(Line {
                start: Point::from_str(s)?,
                end: Point::from_str(e)?,
            }),
            _ => Err("error parsing Line".to_string()),
        }
    }
}

impl Line {
    fn get_distinct_hortogonal_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        if self.start.x == self.end.x {
            for y in min(self.start.y, self.end.y)..=max(self.start.y, self.end.y) {
                points.push(Point { x: self.start.x, y })
            }
        }

        if self.start.y == self.end.y {
            for x in min(self.start.x, self.end.x)..=max(self.start.x, self.end.x) {
                points.push(Point { y: self.start.y, x })
            }
        }

        points
    }

    fn get_distinct_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        if self.start.x == self.end.x {
            for y in min(self.start.y, self.end.y)..=max(self.start.y, self.end.y) {
                points.push(Point { x: self.start.x, y })
            }
        } else if self.start.y == self.end.y {
            for x in min(self.start.x, self.end.x)..=max(self.start.x, self.end.x) {
                points.push(Point { y: self.start.y, x })
            }
        } else {
            let dist = (self.end.x - self.start.x).abs();

            let x_asc = self.start.x < self.end.x;
            let y_asc = self.start.y < self.end.y;

            for i in 0..=dist {
                points.push(Point {
                    y: self.start.y + if y_asc { i } else { -i },
                    x: self.start.x + if x_asc { i } else { -i },
                })
            }
        }

        points
    }
}
