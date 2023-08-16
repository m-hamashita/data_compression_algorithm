use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone)]
enum HuffmanNode {
    Internal {
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
    Leaf {
        character: char,
        frequency: usize,
    },
}

impl HuffmanNode {
    fn frequency(&self) -> usize {
        match self {
            HuffmanNode::Leaf { frequency, .. } => *frequency,
            HuffmanNode::Internal { left, right } => left.frequency() + right.frequency(),
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency().cmp(&self.frequency())
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_tree(frequencies: &HashMap<char, usize>) -> HuffmanNode {
    let mut heap = BinaryHeap::new();

    for (character, &frequency) in frequencies {
        heap.push(HuffmanNode::Leaf {
            character: *character,
            frequency,
        });
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let internal = HuffmanNode::Internal {
            left: Box::new(left),
            right: Box::new(right),
        };
        heap.push(internal);
    }

    heap.pop().unwrap()
}

fn build_codebook(
    node: &HuffmanNode,
    prefix: VecDeque<bool>,
    codebook: &mut HashMap<char, VecDeque<bool>>,
) {
    match node {
        HuffmanNode::Leaf { character, .. } => {
            codebook.insert(*character, prefix);
        }
        HuffmanNode::Internal { left, right } => {
            let mut left_prefix = prefix.clone();
            left_prefix.push_back(false);
            build_codebook(left, left_prefix, codebook);

            let mut right_prefix = prefix;
            right_prefix.push_back(true);
            build_codebook(right, right_prefix, codebook);
        }
    }
}

fn main() {
    let text = "ABRACADABRAABRACADABRA";
    let mut frequencies: HashMap<char, usize> = HashMap::new();

    for ch in text.chars() {
        let counter = frequencies.entry(ch).or_insert(0);
        *counter += 1;
    }

    let tree = build_tree(&frequencies);
    let mut codebook = HashMap::new();
    build_codebook(&tree, VecDeque::new(), &mut codebook);

    for (char, code) in codebook.iter() {
        let code_str: String = code
            .iter()
            .map(|&bit| if bit { '1' } else { '0' })
            .collect();
        println!("Character: '{}', Code: {}", char, code_str);
    }

    let mut huffman_encoded = VecDeque::new();
    for ch in text.chars() {
        let code = codebook.get(&ch).expect("Character not in codebook");
        huffman_encoded.extend(code.clone());
    }
    println!("Huffman encoded: {:?}", huffman_encoded);

    println!("Original size: {} bits", text.len() * 8);
    println!("Compressed size: {} bits", huffman_encoded.len());
}
