#![allow(unused)]
use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash, rc::Rc};


pub struct Graph<T> {
    // We have a set of nodes called Vertex set
    // And each node can point to any other node in the vertex set
    // the pair (u, v) means from u to v there is an edge
    // We need to store key value pair
    // edges.get(1) = vec![2,3,4] it means there are 3 edges namely
    // (1, 2), (1, 3) and (1, 4)
    edges: HashMap<Rc<T>, HashSet<Rc<T>>>,
}

impl<T: Eq + PartialEq + Hash> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph { edges: HashMap::new() }
    }

    pub fn vertices(&self) -> Vec<Rc<T>> {
        self.edges.keys().into_iter().map(|x| x.clone()).collect()
    }

    pub fn insert_vertex(&mut self, u: T) {
        self.edges.insert(Rc::new(u), HashSet::new());
    }

    pub fn insert_edge(&mut self, u: T, v: T) {
        // node u can already be in the HashMap or it is not in the HashMap
        let u_ref = Rc::new(u);
        let v_ref = Rc::new(v);
        if self.edges.contains_key(&u_ref) {
            let neighbors = self.edges.get_mut(&u_ref).unwrap();
            neighbors.insert(v_ref);
        } else {
            let mut neighbors = HashSet::new();
            neighbors.insert(v_ref);
            self.edges.insert(u_ref, neighbors);
        }
    }

    pub fn remove_edge(&mut self, u: &T, v: &T) {
        self.edges.get_mut(u).unwrap().remove(v);
    }

    pub fn remove_vertex(&mut self, u: &T) {
        self.edges.remove(u);
    }

    pub fn contains_vertex(&self, u: &T) -> bool {
        if self.edges.contains_key(u) {
            true
        } else {
            for (_, neighbors) in self.edges.iter() {
                if neighbors.contains(u) {
                    return true;
                }
            }
            false
        }
    }

    pub fn contains_edge(&mut self, u: &T, v: &T) -> bool {
        if self.edges.contains_key(u) {
            self.edges.get(u).unwrap().contains(v)
        } else {
            false
        }
    }

    pub fn neighbors(&self, u: &T) -> Vec<Rc<T>> {
        self.edges.get(u).unwrap().iter().cloned().collect()
    }

    pub fn path_exists_between(&self, u: &T, v: &T) -> bool {
        // Use bfs or dfs
        // bfs requires a queue data structure refer https://doc.rust-lang.org/std/collections/struct.VecDeque.html
        // dfs requires recursion
        // in both cases keep track of visited nodes using HashSet
        let mut visited: HashSet<&T> = HashSet::new();

        // Popping in the front is not so efficient in Vec

        let mut queue: VecDeque<&T> = VecDeque::new();
        queue.push_back(u);

        while let Some(x) = queue.pop_front() {
            visited.insert(x);
            if x == v {
                return true;
            }
            if let Some(neighbors) = self.edges.get(x) {
                for neighbor in neighbors {
                    // as_ref it takes you from &Rc<T> to &T
                    // Refer AsRef trait
                    if !visited.contains(neighbor.as_ref()) {
                        queue.push_back(&neighbor);
                    }
                }
            }
        }
        false
    }
}

// Write your own tests if needed

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck};

    #[test]
    fn property_add_vertex_contains_vertex() {
        fn prop(val: i32) -> bool {
            let mut graph = Graph::new();
            graph.insert_vertex(val);
            graph.contains_vertex(&val)
        }
        QuickCheck::new().quickcheck(prop as fn(i32) -> bool);
    }

    #[test]
    fn property_add_edge_contains_edge() {
        fn prop(val1: i32, val2: i32) -> bool {
            let mut graph = Graph::new();
            graph.insert_edge(val1, val2);
            graph.contains_edge(&val1, &val2)
        }
        QuickCheck::new().quickcheck(prop as fn(i32, i32) -> bool);
    }

    #[test]
    fn edge_not_present_after_removal() {
        fn prop(u: i32, v: i32) -> bool {
            let mut graph = Graph::new();
            graph.insert_edge(u.clone(), v.clone());
            graph.remove_edge(&u, &v);
            !graph.contains_edge(&u, &v)
        }
        QuickCheck::new().quickcheck(prop as fn(i32, i32) -> bool);
    }

    #[test]
    fn vertex_not_present_after_removal() {
        fn prop(val: i32) -> bool {
            let mut graph = Graph::new();
            graph.insert_vertex(val.clone());
            graph.remove_vertex(&val);
            !graph.contains_vertex(&val)
        }
        QuickCheck::new().quickcheck(prop as fn(i32) -> bool);
    }

    #[test]
    fn direct_path_exists() {
        let mut graph = Graph::new();
        graph.insert_edge("A", "B");
        assert!(graph.path_exists_between(&"A", &"B"));
    }

    // Test that path_exists_between returns true for indirectly connected vertices
    #[test]
    fn indirect_path_exists() {
        let mut graph = Graph::new();
        graph.insert_edge("A", "B");
        graph.insert_edge("B", "C");
        assert!(graph.path_exists_between(&"A", &"C"));
    }

    // Test that path_exists_between returns false when no path exists
    #[test]
    fn no_path_exists() {
        let mut graph = Graph::new();
        graph.insert_edge("A", "B");
        graph.insert_edge("C", "D");
        assert!(!graph.path_exists_between(&"A", &"C"));
    }

    // Test that path_exists_between returns true for a complex graph where a path exists
    #[test]
    fn complex_graph_with_path() {
        let mut graph = Graph::new();
        graph.insert_edge("A", "B");
        graph.insert_edge("B", "C");
        graph.insert_edge("C", "D");
        graph.insert_edge("D", "E");
        graph.insert_edge("A", "F");
        graph.insert_edge("F", "G");
        graph.insert_edge("G", "D");
        assert!(graph.path_exists_between(&"A", &"E"));
    }

    // Test that path_exists_between returns false for a complex graph where no path exists
    #[test]
    fn complex_graph_without_path() {
        let mut graph = Graph::new();
        graph.insert_edge("A", "B");
        graph.insert_edge("B", "C");
        graph.insert_edge("E", "F");
        graph.insert_edge("F", "G");
        assert!(!graph.path_exists_between(&"A", &"G"));
    }

    #[test]
    fn test_contains_vertex() {
        let mut graph = Graph::new();
        graph.insert_edge("A", "B");
        graph.insert_edge("C", "B");

        assert!(graph.contains_vertex(&"A"));
        assert!(graph.contains_vertex(&"B"));
        assert!(graph.contains_vertex(&"C"));
    }
}

