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

    // ヒープに追加することで、最小値が常に先頭に来る
    for (character, &frequency) in frequencies {
        heap.push(HuffmanNode::Leaf {
            character: *character,
            frequency,
        });
    }

    // 最小の2つを取り出して、それらを子とするノードを作成し、ヒープに追加する
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
    // huffman tree を走査して、codebook を作成する
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

fn huffman_encode(text: &str, codebook: &HashMap<char, VecDeque<bool>>) -> VecDeque<bool> {
    let mut encoded = VecDeque::new();
    for ch in text.chars() {
        let code = codebook.get(&ch).expect("Character not in codebook");
        encoded.extend(code.clone());
    }
    encoded
}

fn huffman_decode(data: &VecDeque<bool>, codebook: &HashMap<char, VecDeque<bool>>) -> String {
    let mut decoded = String::new();
    let mut bits = VecDeque::new();

    for bit in data.iter() {
        bits.push_back(*bit);
        if let Some(&ch) = codebook
            .iter()
            .find(|&(_, value)| value == &bits)
            .map(|(key, _)| key)
        {
            decoded.push(ch);
            bits.clear();
        }
    }
    decoded
}


fn main() {
    let input = "ABRACADABRAABRACADABRA";
    let mut frequencies: HashMap<char, usize> = HashMap::new();

    for char in input.chars() {
        let counter = frequencies.entry(char).or_insert(0);
        *counter += 1;
    }

    let huffman_tree = build_tree(&frequencies);
    let mut codebook = HashMap::new();
    build_codebook(&huffman_tree, VecDeque::new(), &mut codebook);

    for (char, code) in codebook.iter() {
        let code_str: String = code
            .iter()
            .map(|&bit| if bit { '1' } else { '0' })
            .collect();
        println!("Character: '{}', Code: {}", char, code_str);
    }

    let huffman_encoded = huffman_encode(input, &codebook);
    let huffman_decoded = huffman_decode(&huffman_encoded, &codebook);
    assert_eq!(input, huffman_decoded);

    println!("Original: {}", input);
    println!("Huffman encoded: {:?}", huffman_encoded);
    println!("Huffman decoded: {}", huffman_decoded);

    println!("Original size: {} bits", input.len() * 8);
    println!("Compressed size: {} bits", huffman_encoded.len());
}
