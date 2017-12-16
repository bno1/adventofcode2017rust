extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashMap;

// Holds the regexes needed to parse input
struct LineParser {
    re_prog: Regex,
    re_children: Regex,
}

// Intermediary representation of a node, used for building the tree
struct NodeInter<'a> {
    prog: &'a str,
    parent: &'a str,
    weight: u32,
    children: Vec<&'a str>,
}

// Tree node
struct Node<'a> {
    prog: &'a str,
    weight: u32,
    weight_tree: u32,
    children: Vec<Node<'a>>,
}

fn empty_node_iter(prog: &str) -> NodeInter {
    NodeInter {
        prog: prog,
        parent: "",
        weight: 0,
        children: Vec::new(),
    }
}

impl LineParser {
    pub fn new() -> Result<LineParser, regex::Error> {
        let re_prog =
            //                  prog name      weight     children list
            match Regex::new(r"([a-zA-Z]+)\s*\((\d+)\)\s*(->\s*(.*)\s*)?")
        {
            Ok(re) => re,
            Err(e) => return Err(e)
        };

        // matches the children names
        let re_children = match Regex::new(r"[a-zA-Z]+") {
            Ok(re) => re,
            Err(e) => return Err(e),
        };

        Ok(LineParser {
            re_prog: re_prog,
            re_children: re_children,
        })
    }

    pub fn parse<'a>(&self, line: &'a str) -> NodeInter<'a> {
        let captures = self.re_prog.captures(line).unwrap();

        let prog = captures.get(1).unwrap().as_str();
        let weight = captures.get(2).unwrap().as_str().parse().unwrap();

        let children = match captures.get(4) {
            Some(r) => self.re_children
                .find_iter(r.as_str())
                .map(|m| m.as_str())
                .collect(),
            None => Vec::new(),
        };

        NodeInter {
            prog: prog,
            parent: "",
            weight: weight,
            children: children,
        }
    }
}

type NodeMap<'a> = HashMap<&'a str, NodeInter<'a>>;

fn build_tree_rec<'a>(map: &NodeMap<'a>, root_prog: &'a str) -> Node<'a> {
    let node = map.get(root_prog).unwrap();

    let children: Vec<Node<'a>> = node.children
        .iter()
        .map(|c| build_tree_rec(map, c))
        .collect();
    let subtree_weight: u32 = children.iter().map(|c| c.weight_tree).sum();

    Node {
        prog: root_prog,
        weight: node.weight,
        weight_tree: node.weight + subtree_weight,
        children: children,
    }
}

fn build_tree<'a, T>(lst: T) -> Option<Node<'a>>
where
    T: Iterator<Item = &'a NodeInter<'a>>,
{
    let mut map: NodeMap<'a> = HashMap::new();

    for node in lst {
        {
            let n: &mut NodeInter<'a> = map.entry(node.prog)
                .or_insert_with(|| empty_node_iter(node.prog));
            n.weight = node.weight;
            n.children = node.children.clone();
        }

        for child in &node.children {
            let c: &mut NodeInter<'a> = map.entry(child)
                .or_insert_with(|| empty_node_iter(child));
            c.parent = node.prog;
        }
    }

    for node in map.values() {
        if node.parent == "" {
            return Some(build_tree_rec(&map, node.prog));
        }
    }

    None
}

fn find_unbalanced<'a>(root: &'a Node<'a>) -> Option<(i32, &'a Node<'a>)> {
    let mut map: HashMap<u32, u32> = HashMap::new();

    for child in &root.children {
        let r = find_unbalanced(child);
        if r.is_some() {
            return r;
        }

        let p = map.entry(child.weight_tree).or_insert(0);
        *p += 1;
    }

    if map.len() > 2 {
        panic!("Unexpected weights");
    } else if map.len() == 2 {
        type Part<'a> = (Vec<(&'a u32, &'a u32)>, Vec<(&'a u32, &'a u32)>);
        let (outliers, inliers): Part = map
            .iter()
            .partition(|&(_, &v)| v == 1);

        if outliers.len() != 1 || inliers.len() != 1 {
            panic!("Unexpected weights");
        }

        let &expect_w = inliers[0].0;
        let &actual_w = outliers[0].0;

        for child in &root.children {
            if child.weight_tree == actual_w {
                return Some((expect_w as i32 - actual_w as i32, child));
            }
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let line_parser = LineParser::new().unwrap();

    let input_raw: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let input: Vec<NodeInter> = input_raw
        .iter()
        .map(|l| line_parser.parse(l))
        .collect();

    let root = build_tree(input.iter()).unwrap();
    println!("Solution 1: {}", root.prog);

    let n = find_unbalanced(&root).unwrap();
    println!("Solution 2: {}", n.0 + n.1.weight as i32);
}
