use crate::grid::{Grid, GridPosition};
use crate::snek::Snek;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use rpds;

struct PathElement {
    priority: f32,
    path: rpds::List<Snek>,
}

impl PartialEq for PathElement {
    fn eq(&self, other: &PathElement) -> bool {
        self.priority == other.priority
    }
}
impl Eq for PathElement {}

impl Ord for PathElement {
    fn cmp(&self, other: &PathElement) -> Ordering {
        if other.priority > self.priority {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

impl PartialOrd for PathElement {
    fn partial_cmp(&self, other: &PathElement) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_not_occupied(snek: &Snek, grid: &Grid) -> GridPosition {
    for i in 0..grid.size_x {
        for j in 0..grid.size_y {
            let p = GridPosition { x: i, y: j };
            if snek.is_occupied(&p) {
                return p;
            }
        }
    }

    panic!("position not found")
}

fn distance_sqared(a: GridPosition, b: GridPosition) -> i32 {
    (a.x - b. x) * (a.x - b. x) + (a.y - b. y) * (a.y - b. y)
}

fn is_blocking(snek: &Snek, grid: &Grid) -> bool {
    let mut visited: HashSet<GridPosition> = HashSet::new();

    let start = find_not_occupied(snek, grid);
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back(start);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        grid.for_each_neighbour(current, |next_position| {
            if !visited.contains(&next_position) && !snek.is_occupied(&next_position) {
                queue.push_back(*next_position);
                visited.insert(*next_position);
            }
        })
    }

    visited.len() - 1 != (grid.size_x * grid.size_y) as usize - snek.body.len()
}

fn can_move(snek: &Snek, grid: &Grid) -> bool {
    let mut can = false;

    grid.for_each_neighbour(snek.get_head(), |position| {
        if !snek.is_occupied(position) {
            can = true;
        }
    });
    return can;
}

pub fn find_path(
    snek: Snek,
    grid: &Grid,
    food: GridPosition,
) -> (rpds::List<Snek>, HashMap<GridPosition, i32>) {
    let mut queue = BinaryHeap::new();
    let list = rpds::List::new();
    let mut scores: HashMap<GridPosition, i32> = HashMap::new();

    queue.push(PathElement {
        priority: 0.0,
        path: list.push_front(snek),
    });

    while !queue.is_empty() {

        
        let el = queue.pop().unwrap();
        let current_snek = el.path.first().unwrap();
        let head = current_snek.get_head();

        if head == food {
            return (el.path.reverse(), scores);

            // if is_blocking(&current_snek, grid) || !can_move(current_snek, grid) {
            //     // println!("return blocking, len {}", queue.len().to_string());
            //     continue;
            // } else {
            //     return (el.path.reverse(), scores);
            // }
        }

        scores.entry(head).and_modify(|e| *e = *e + 1).or_insert(1);

        grid.for_each_neighbour(head, |next_position| {
            if current_snek.is_occupied(next_position) {
                return;
            }

            let next_snek;
            if *next_position == food {
                next_snek = current_snek.eat(*next_position);
            } else {
                next_snek = current_snek.go(next_position);
            }


            // let mut score = 0;
            // if is_blocking(&next_snek, grid) {
            //     score += 1000000;
            //     // println!("blocking");
            // }

            queue.push(PathElement {
                priority: 
                // (*scores.entry(*next_position).or_insert(1) as f32), // + (distanceSquared(head, next_position) as f32)
                (distance_sqared(food, next_snek.get_head())) as f32,
                path: el.path.push_front(next_snek),
            })
        });
    }
    panic!("path not found")
    // rpds::List::new()
}
