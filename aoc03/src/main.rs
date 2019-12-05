use std::collections::HashSet;
use std::error;
use std::io;
use std::io::prelude::*;


#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct WirePath {
    directions: Vec<Direction>,
    lengths: Vec<i32>,
}

impl WirePath {

    fn new() -> WirePath {
        return WirePath{ directions: Vec::new(), lengths: Vec::new() }
    }

    fn coordinates(&self) -> HashSet<(i32, i32)> {
        let mut coords = HashSet::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut dx;
        let mut dy;

        for (d, l) in self.directions.iter().zip(self.lengths.iter()) {
            match d {
                Direction::Right => { dx = 1; dy = 0 },
                Direction::Left => { dx = -1; dy = 0 },
                Direction::Up => { dx = 0; dy = 1 },
                Direction::Down => { dx = 0; dy = -1 },
            }

            for _ in 0..*l {
                x += dx;
                y += dy;
                coords.insert((x, y));
            }
        }

        return coords
    }

}

fn parse_input() -> Result<Vec<WirePath>, Box<error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut buf)?;
    let input: Vec<&str> = buf.trim().split('\n').collect();

    let mut wire_paths = Vec::new();
    for line in input {
        let tokens: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
        let mut wp = WirePath::new();
        for t in tokens {
            let (head, tail) = t.split_at(1);
            match head {
                "R" => wp.directions.push(Direction::Right),
                "L" => wp.directions.push(Direction::Left),
                "U" => wp.directions.push(Direction::Up),
                "D" => wp.directions.push(Direction::Down),
                 _ => unreachable!(),
            }

            let l: i32 = tail.parse()?;
            wp.lengths.push(l);
        }

        wire_paths.push(wp);
    }

    Ok(wire_paths)
}

fn find_min_dist(wire_paths: &Vec<WirePath>) -> Option<i32> {
    assert!(wire_paths.len() == 2);

    let wp1 = &wire_paths[0];
    let wp2 = &wire_paths[1];

    let c1 = wp1.coordinates();
    let c2 = wp2.coordinates();
    let overlaps = c1.intersection(&c2);
    return overlaps.map(|(x,y)| x.abs() + y.abs()).min()
}


fn main() {
    let wire_paths = parse_input().unwrap();
    let min_dist = find_min_dist(&wire_paths);
    println!("{:?}", min_dist);
}
