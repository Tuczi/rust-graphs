use std::collections::{HashSet, VecDeque};
use std::slice::Iter;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Edge<'a> {
    pub weight: u32,
    pub destination_node: &'a Node<'a>,
}

impl<'a> Edge<'a> {
    pub fn new(weight: u32, destination_node: &'a Node<'a>) -> Edge {
        Edge {
            weight,
            destination_node,
        }
    }
}

// Should use better `PartialEq` and `Hash` implementation (id based only)
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Node<'a> {
    pub id: u32,
    pub children: Vec<Edge<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(id: u32, children: Vec<Edge<'a>>) -> Node {
        Node { id, children }
    }

    pub fn children_edges_iter(&self) -> Iter<'_, Edge<'a>> {
        self.children.iter()
    }
}

pub trait NodeQueue<'a> {
    fn enqueue(&mut self, node: &'a Node<'a>);
    fn dequeue(&mut self) -> Option<&'a Node<'a>>;
}

#[derive(Default)]
struct FifoNodeQueue<'a> {
    data: VecDeque<&'a Node<'a>>,
}

impl<'a> NodeQueue<'a> for FifoNodeQueue<'a> {
    fn enqueue(&mut self, node: &'a Node<'a>) {
        self.data.push_back(node)
    }

    fn dequeue(&mut self) -> Option<&'a Node<'a>> {
        self.data.pop_front()
    }
}

#[derive(Default)]
struct LifoNodeQueue<'a> {
    data: VecDeque<&'a Node<'a>>,
}

impl<'a> NodeQueue<'a> for LifoNodeQueue<'a> {
    fn enqueue(&mut self, node: &'a Node<'a>) {
        self.data.push_back(node)
    }

    fn dequeue(&mut self) -> Option<&'a Node<'a>> {
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
                let child = edge.destination_node;
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
        let n5 = Node::new(5, vec![]);
        let n4 = Node::new(4, vec![Edge::new(1, &n5)]);
        let n3 = Node::new(3, vec![Edge::new(1, &n4), Edge::new(1, &n5)]);
        let n2 = Node::new(2, vec![Edge::new(1, &n4), Edge::new(1, &n3)]);
        let n1 = Node::new(1, vec![Edge::new(1, &n2), Edge::new(1, &n3)]);

        let graph = Graph {};

        println!("DFS");
        graph.dfs(&n1, |node| println!("Visited {:?}", node));

        println!("BFS");
        graph.bfs(&n1, |node| println!("Visited {:?}", node));
    }
}
