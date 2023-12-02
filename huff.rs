use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::fmt;

#[derive(Eq, Clone)]
struct Node {
    o : u32,
    c : Option<String>,
    leftnode : Box<Option<Node>>,
    rightnode : Box<Option<Node>>,
}

// implementing traits to compare nodes by frequency, disregarding the character (Ord implies PartialOrd and Eq, which in turn implies PartialEq)
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.o.cmp(&other.o)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.o == other.o
    }
}

// implementing display for node type
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node [{} : {}]", self.c.clone().unwrap(), self.o)
    }
}

fn get_huff_tree(text: &str) -> Node{

    let mut occurances = BTreeMap::new();
    let mut huff_tree = Vec::new();

    //Get characters and occurances
    for c in text.chars() {
        occurances.entry(c.to_string()).and_modify(|e| *e += 1).or_insert(1);
    }

    //Convert to heap of nodes
    for (k,v) in occurances {
        huff_tree.push(Node{ c : Some(k), o : v, leftnode : Box::new(None), rightnode : Box::new(None)});
    }
    
    while huff_tree.len() > 1 {
        huff_tree.sort_by(|a, b| b.cmp(a)); // Sort descending
        // Pop left and right node
        let left = huff_tree.pop(); 
        let right = huff_tree.pop();
        let nodefreq = left.clone().unwrap().o + right.clone().unwrap().o;
        
        huff_tree.push(Node{ c : Some("/".to_string()), o : nodefreq, leftnode : Box::new(left.clone()), rightnode : Box::new(right.clone())});

        println!("{} {}", left.unwrap(), right.clone().unwrap());
    }

    huff_tree.pop().unwrap()
}


fn main() {
    let root = get_huff_tree("sadfasdfewqfgdxfgvadsfkleawsofsiaeudjfkaya7swefrusgdftvyay7sfraewfgsadfagy7erqwfb"); 
    println!("{}", root)
}


