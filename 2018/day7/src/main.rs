#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|s| parse_to_step(s))
        .collect::<Result<Vec<[char; 2]>, _>>()?;
    let mut tree = Node::new();
    for step in input.into_iter() {
        tree.insert(step);
    }
    // For part 1, the number of workers is 1 and the processing time
    // of each job is 0
    tree.processing_time(5);
    Ok(())
}

#[derive(Debug)]
struct Node {
    job: Option<char>,
    released: RefCell<u32>,
    finished: RefCell<u32>,
    parents: RefCell<Vec<Weak<Self>>>,
    children: RefCell<Vec<Rc<Self>>>,
}

impl Node {
    fn new() -> Self {
        Self {
            job: None,
            released: RefCell::new(0),
            finished: RefCell::new(0),
            parents: RefCell::new(Vec::new()),
            children: RefCell::new(Vec::new()),
        }
    }

    fn job_processing_time(&self) -> u32 {
        self.job.unwrap() as u32 - 4
    }

    fn insert(&mut self, jobs: [char; 2]) {
        let from_job = jobs[0];
        let to_job = jobs[1];
        let parent = self.breadth_first_search(from_job);
        let child = self.breadth_first_search(to_job);

        match parent {
            Some(p_node) => match child {
                Some(c_node) => {
                    (*p_node.children.borrow_mut()).push(Rc::clone(&c_node));
                    (*c_node.parents.borrow_mut()).push(Rc::downgrade(&p_node));
                    self.children
                        .borrow_mut()
                        .retain(|node| node.job.unwrap() != to_job);
                }
                None => {
                    let c_node = Rc::new(Node {
                        job: Some(to_job),
                        released: RefCell::new(0),
                        finished: RefCell::new(0),
                        parents: RefCell::new(vec![Rc::downgrade(&p_node)]),
                        children: RefCell::new(Vec::new()),
                    });
                    (*p_node.children.borrow_mut()).push(Rc::clone(&c_node));
                }
            },
            None => match child {
                Some(c_node) => {
                    let p_node = Rc::new(Node {
                        job: Some(from_job),
                        released: RefCell::new(0),
                        finished: RefCell::new(0),
                        parents: RefCell::new(Vec::new()),
                        children: RefCell::new(vec![Rc::clone(&c_node)]),
                    });
                    (*self.children.borrow_mut()).push(Rc::clone(&p_node));
                    (*c_node.parents.borrow_mut()).push(Rc::downgrade(&p_node));
                    self.children
                        .borrow_mut()
                        .retain(|node| node.job.unwrap() != to_job);
                }
                None => {
                    let p_node = Rc::new(Node {
                        job: Some(from_job),
                        released: RefCell::new(0),
                        finished: RefCell::new(0),
                        parents: RefCell::new(Vec::new()),
                        children: RefCell::new(Vec::new()),
                    });
                    let c_node = Rc::new(Node {
                        job: Some(to_job),
                        released: RefCell::new(0),
                        finished: RefCell::new(0),
                        parents: RefCell::new(vec![Rc::downgrade(&p_node)]),
                        children: RefCell::new(Vec::new()),
                    });
                    (*p_node.children.borrow_mut()).push(Rc::clone(&c_node));
                    (*self.children.borrow_mut()).push(Rc::clone(&p_node));
                }
            },
        }
    }

    fn breadth_first_search(&self, job: char) -> Option<Rc<Node>> {
        let mut q: VecDeque<Rc<Node>> = VecDeque::new();
        for n in self.children.borrow().iter() {
            q.push_back(Rc::clone(&n));
        }

        while let Some(node) = q.pop_front() {
            if node.job.unwrap() == job {
                return Some(node);
            }
            for n in node.children.borrow().iter() {
                q.push_back(Rc::clone(&n));
            }
        }
        None
    }

    fn check_precedence(&self, v: &[char]) -> bool {
        for p in self.parents.borrow().iter() {
            if !v.contains(&p.upgrade().unwrap().job.unwrap()) {
                return false;
            };
        }
        true
    }

    fn update_release(&self) {
        let mut max_released: u32 = 0;
        let mut released: u32;
        for p in self.parents.borrow().iter() {
            released = *p.upgrade().unwrap().finished.borrow();
            if released > max_released {
                max_released = released;
            }
        }
        *self.released.borrow_mut() = max_released;
    }

    fn processing_time(&self, workers: usize) {
        let mut workers = vec![0; workers];
        let mut released: u32;
        let mut processing: u32;
        let mut q: VecDeque<Rc<Node>> = VecDeque::new();
        let mut v = Vec::new();
        for n in self.children.borrow().iter() {
            q.push_back(Rc::clone(n));
        }
        q.make_contiguous()
            .sort_by(|a, b| a.job.unwrap().cmp(&b.job.unwrap()));

        while let Some(node) = q.pop_front() {
            if v.contains(&node.job.unwrap()) {
                continue;
            }
            released = *node.released.borrow();
            processing = node.job_processing_time();
            if workers[0] >= released {
                workers[0] += processing;
            } else {
                workers[0] = released + processing;
            }
            *node.finished.borrow_mut() = workers[0];
            v.push(node.job.unwrap());
            for n in node.children.borrow().iter() {
                if n.check_precedence(&v) {
                    n.update_release();
                    q.push_back(Rc::clone(&n));
                }
            }
            q.make_contiguous()
                .sort_by(|a, b| a.job.unwrap().cmp(&b.job.unwrap()));
            q.make_contiguous()
                .sort_by(|a, b| a.released.borrow().cmp(&b.released.borrow()));

            workers.sort();
        }
        dbg!(v.into_iter().collect::<String>());
        dbg!(workers[workers.len() - 1]);
    }
}

fn parse_to_step(s: &str) -> anyhow::Result<[char; 2]> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Step ([[:upper:]]) must be finished before step ([[:upper:]]) can begin.")
                .unwrap();
    }
    let jobs = match RE.captures(s) {
        Some(jobs) => jobs,
        None => return Err(anyhow::anyhow!("Unrecognized dependency")),
    };
    Ok([jobs[1].as_bytes()[0] as char, jobs[2].as_bytes()[0] as char])
}
