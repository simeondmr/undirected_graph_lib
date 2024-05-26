#[allow(dead_code)]
mod undirected_graph {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::fmt::Debug;
    use std::rc::{Rc, Weak};

    pub struct Node<T: Debug> {
        vertex_name: i32,
        value: T,
        neighbors: Vec<Weak<RefCell<Self>>>
    }

    impl<T: Debug> Node<T> {
        pub fn new(vertex_name: i32, value: T) -> Rc<RefCell<Node<T>>> {
            Rc::new(RefCell::new(Node {
                vertex_name,
                value,
                neighbors: Vec::new(),
            }))
        }

        pub fn vertex_name(&self) -> i32 {
            self.vertex_name
        }

        pub fn value(&self) -> &T {
            &self.value
        }

        pub fn set_vertex_name(&mut self, vertex_name: i32) {
            self.vertex_name = vertex_name;
        }

        pub fn neighbors(&self) -> &Vec<Weak<RefCell<Self>>> {
            &self.neighbors
        }

    }

    impl<T: Debug> PartialEq for Node<T> {
        fn eq(&self, other: &Self) -> bool {
            self.vertex_name == other.vertex_name
        }
    }

    pub struct Graph<T: Debug> {
        nodes: Vec<Rc<RefCell<Node<T>>>>
    }

    impl<T: Debug> Graph<T> {
        pub fn new() -> Graph<T> {
            Graph {
                nodes: Vec::new(),
            }
        }

        pub fn add_node(&mut self, node: Rc<RefCell<Node<T>>>) {
            self.nodes.push(node);
        }

        pub fn add_edge(node0: Rc<RefCell<Node<T>>>, node1: Rc<RefCell<Node<T>>>) {
            node0.borrow_mut().neighbors.push(Rc::downgrade(&node1));
            node1.borrow_mut().neighbors.push(Rc::downgrade(&node0));
        }

        pub fn nodes(&self) -> &Vec<Rc<RefCell<Node<T>>>> {
            &self.nodes
        }

        pub fn bfs(&self, start: Rc<RefCell<Node<T>>>) -> Vec<Rc<RefCell<Node<T>>>> {
            let mut visited = Vec::new();
            let mut queue = VecDeque::new();

            visited.push(Rc::clone(&start));
            queue.push_back(Rc::clone(&start));

            while let Some(current) = queue.pop_front() {
                let current_borrow = current.borrow();
                println!("Vertex: {:?}, Value: {:?}", current_borrow.vertex_name(), current_borrow.value());
                for neighbor in &current.borrow().neighbors {
                    if let Some(strong_neighbor) = neighbor.upgrade() {
                        if !visited.contains(&strong_neighbor) {
                            queue.push_back(Rc::clone(&strong_neighbor));
                            visited.push(strong_neighbor);
                        }
                    }
                }
            }
            visited
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::rc::Rc;
    use crate::undirected_graph::{Graph, Node};

    #[derive(Debug)]
    struct Data {
        a: i32,
        b: i32
    }

    #[test]
    fn bfs_test() {
        let node0 = Node::new(0, Data { a: 1, b: 2 });
        let node1 = Node::new(1, Data { a: 3, b: 4 });
        let node2 = Node::new(2, Data { a: 5, b: 6 });
        let node3 = Node::new(3, Data { a: 7, b: 8 });

        let mut graph = Graph::new();
        graph.add_node(Rc::clone(&node0));
        graph.add_node(Rc::clone(&node1));
        graph.add_node(Rc::clone(&node2));
        graph.add_node(Rc::clone(&node3));

        Graph::add_edge(Rc::clone(&node0), Rc::clone(&node2));
        Graph::add_edge(Rc::clone(&node0), Rc::clone(&node3));
        Graph::add_edge(Rc::clone(&node1), Rc::clone(&node2));
        Graph::add_edge(Rc::clone(&node2), Rc::clone(&node3));

        let nodes = graph.nodes();
        let visited= graph.bfs(node0);

        nodes.iter().for_each(move |node| {
            assert_eq!(visited.contains(node), true);
        });
    }
}
