use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Edge {
    pub weight: u32,
    pub destination_node: Rc<Node>,
}

impl Edge {
    pub fn new(weight: u32, destination_node: Rc<Node>) -> Edge {
        Edge {
            weight,
            destination_node,
        }
    }
}

// Should use better `PartialEq` and `Hash` implementation (id based only)
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Node {
    pub id: u32,
    pub children: Vec<Edge>,
}

impl Node {
    pub fn new(id: u32, children: Vec<Edge>) -> Node {
        Node { id, children }
    }

    pub fn children_edges_iter(&self) -> Iter<'_, Edge> {
        self.children.iter()
    }
}

pub trait NodeQueue<'a> {
    fn enqueue(&mut self, node: &'a Node);
    fn dequeue(&mut self) -> Option<&'a Node>;
}

#[derive(Default)]
struct FifoNodeQueue<'a> {
    data: VecDeque<&'a Node>,
}

impl<'a> NodeQueue<'a> for FifoNodeQueue<'a> {
    fn enqueue(&mut self, node: &'a Node) {
        self.data.push_back(node)
    }

    fn dequeue(&mut self) -> Option<&'a Node> {
        self.data.pop_front()
    }
}

#[derive(Default)]
struct LifoNodeQueue<'a> {
    data: VecDeque<&'a Node>,
}

impl<'a> NodeQueue<'a> for LifoNodeQueue<'a> {
    fn enqueue(&mut self, node: &'a Node) {
        self.data.push_back(node)
    }

    fn dequeue(&mut self) -> Option<&'a Node> {
        self.data.pop_back()
    }
}

struct Graph {}

impl Graph {
    pub fn dfs<F>(&self, start: &Node, f: F)
    where
        F: Fn(&Node),
    {
        self.iterate_all(start, LifoNodeQueue::default(), f)
    }

    pub fn bfs<F>(&self, start: &Node, f: F)
    where
        F: Fn(&Node),
    {
        self.iterate_all(start, FifoNodeQueue::default(), f)
    }

    fn iterate_all<'a, F, Q>(&self, start: &'a Node, mut to_visit: Q, f: F)
    where
        F: Fn(&Node),
        Q: NodeQueue<'a>,
    {
        let mut already_visited: HashSet<&Node> = HashSet::new();

        to_visit.enqueue(start);
        already_visited.insert(start);

        while let Some(current_node) = to_visit.dequeue() {
            f(&current_node);

            for edge in current_node.children_edges_iter() {
                let child = edge.destination_node.as_ref();
                if already_visited.insert(child) {
                    to_visit.enqueue(child);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n5 = Rc::new(Node::new(5, vec![]));
        let n4 = Rc::new(Node::new(4, vec![Edge::new(1, n5.clone())]));
        let n3 = Rc::new(Node::new(
            3,
            vec![Edge::new(1, n4.clone()), Edge::new(1, n5.clone())],
        ));
        let n2 = Rc::new(Node::new(
            2,
            vec![Edge::new(1, n4.clone()), Edge::new(1, n3.clone())],
        ));
        let n1 = Rc::new(Node::new(
            1,
            vec![Edge::new(1, n2.clone()), Edge::new(1, n3.clone())],
        ));

        let graph = Graph {};

        println!("DFS");
        graph.dfs(&n1, |node| println!("Visited {:?}", node));

        println!("BFS");
        graph.bfs(&n1, |node| println!("Visited {:?}", node));
    }
}
