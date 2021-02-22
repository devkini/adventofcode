use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::iter::FromIterator;

use regex::Regex;

/// Job is a hack used to implement min-heap with std::collections::BinaryHeap,
/// because default BinaryHeap only supports max-heap.
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Job(char);

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Job) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord,
        }
    }
}

/// 2-way forward and backward dependency indexing
pub struct Dependency {
    forward: HashMap<char, String>,
    backward: HashMap<char, String>,
}

#[derive(Debug, Default, Clone)]
pub struct Worker {
    task: Job,
    remaining: usize,
}

pub fn part1(input: &Dependency) -> String {
    // first, resolve for root nodes
    let a: HashSet<char> = HashSet::from_iter(input.forward.keys().cloned());
    let b = HashSet::from_iter(input.backward.keys().cloned());
    let root = a.difference(&b);

    // construct the available steps
    let mut available = BinaryHeap::from_iter(root.cloned().map(Job));
    let mut sorted = String::with_capacity(input.forward.len());

    while let Some(step) = available.pop() {
        // append to sorted set
        sorted.push(step.0);

        // query for next dependency
        if let Some(next) = input.forward.get(&step.0) {
            // check if the steps are ready
            for c in next.chars() {
                if let Some(requirement) = input.backward.get(&c) {
                    if requirement.chars().all(|c| sorted.contains(c)) {
                        // append to available step
                        available.push(Job(c));
                    }
                }
            }
        }
    }

    sorted
}

pub fn part2(input: &Dependency, num_workers: usize, time_offset: usize) -> usize {
    // first, resolve for root nodes
    let a: HashSet<char> = HashSet::from_iter(input.forward.keys().cloned());
    let b = HashSet::from_iter(input.backward.keys().cloned());
    let root = a.difference(&b);

    // construct the available steps
    let mut available = BinaryHeap::from_iter(root.cloned().map(Job));
    let mut done = String::with_capacity(input.forward.len());

    // begin the simulation
    let mut workers = vec![Worker::default(); num_workers];
    let mut elapsed = 0;

    // keep simulating until we completed all tasks
    while done.len() <= input.forward.len() && elapsed < 10000 {
        // println!("[{}]       pre: {:?}", elapsed, workers);

        // pass 1: assign jobs to all available workers
        for worker in workers.iter_mut() {
            if worker.remaining == 0 {
                if let Some(job) = available.pop() {
                    // get to work people!
                    worker.task = job;
                    worker.remaining = time_offset + (worker.task.0 as u8 - b'A') as usize + 1;
                } else {
                    break;
                }
            }
        }
        // println!("[{}]  assigned: {:?}", elapsed, workers);

        // pass 2: werk werk
        for worker in workers.iter_mut() {
            if worker.remaining > 0 {
                worker.remaining -= 1;

                if worker.remaining == 0 {
                    // mark job as done
                    done.push(worker.task.0);

                    // query for next available job
                    if let Some(next) = input.forward.get(&worker.task.0) {
                        // check if the steps are ready
                        for c in next.chars() {
                            if let Some(requirement) = input.backward.get(&c) {
                                if requirement.chars().all(|c| done.contains(c)) {
                                    // append to available step
                                    available.push(Job(c));
                                }
                            }
                        }
                    }
                }
            }
        }
        // println!("[{}]      post: {:?}", elapsed, workers);
        // println!("[{}] available: {:?}", elapsed, available);

        // time is ticking
        elapsed += 1;
    }

    elapsed
}

pub fn get_input(f: impl Read) -> Result<Dependency, Box<dyn Error>> {
    // read data from input.txt
    let input = BufReader::new(f).lines();

    // parse the dependency
    let re = Regex::new(r"^Step (.) must be finished before step (.) can begin\.$")?;
    let mut forward = HashMap::new();
    let mut backward = HashMap::new();

    for line in input {
        if let Some(parsed) = re.captures(line?.as_str()) {
            let try_parse = |n| -> Result<char, Box<dyn Error>> {
                Ok(parsed
                    .get(n)
                    .ok_or_else(|| "malformed input")?
                    .as_str()
                    .chars()
                    .next()
                    .ok_or_else(|| "unable to read dependency")?)
            };

            // extract the dependency pair
            let before = try_parse(1)?;
            let after = try_parse(2)?;

            // index the dependencies in both forward and inverted order
            forward
                .entry(before)
                .or_insert_with(String::new)
                .push(after);
            backward
                .entry(after)
                .or_insert_with(String::new)
                .push(before);
        }
    }

    Ok(Dependency { forward, backward })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
    "#;

    #[test]
    fn test_part1() {
        let input = get_input(DATA.as_bytes()).expect("unable to parse input");
        assert_eq!("CABDFE", part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_input(DATA.as_bytes()).expect("unable to parse input");
        assert_eq!(15, part2(&input, 2, 0));
    }
}
