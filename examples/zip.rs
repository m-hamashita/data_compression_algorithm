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

const MIN_MATCH: usize = 2;
// The window size is the maximum distance backwards we can refer to
const WINDOW_SIZE: usize = 255;

#[derive(Debug)]
struct Encoded {
    offset: usize,
    length: usize,
    byte: u8,
}

fn find_longest_match(data: &[u8], cur: usize) -> (usize, usize) {
    let mut max_len = 0;
    let mut match_index = 0;

    // Start at the beginning of the window (max(0, cur - WINDOW_SIZE))
    let mut start = cur.saturating_sub(WINDOW_SIZE);

    while start < cur {
        // start から始まる文字列と現在位置(cur)から始まる文字列の最長一致を探す
        let mut reference_match_index = start;
        let mut current_match_index = cur;

        // 一致する文字列を探す
        // 一致する文字列の長さが WINDOW_SIZE を超えないようにする
        while current_match_index < data.len()
            && data[reference_match_index] == data[current_match_index]
            && (current_match_index - cur) < WINDOW_SIZE
        {
            reference_match_index += 1;
            current_match_index += 1;
        }

        // 一致する文字列の長さを計算する
        let len = reference_match_index - start;
        if len > max_len {
            max_len = len;
            match_index = start;
        }

        start += 1;
    }

    // 相対位置と一致する文字列の長さを返す
    (cur - match_index, max_len)
}

fn lz77_encode(data: &[u8]) -> Vec<Encoded> {
    let mut compressed = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let (offset, length) = find_longest_match(data, i);

        // MIN_MATCH より短い一致は圧縮しない
        if length < MIN_MATCH {
            compressed.push(Encoded {
                offset: 0,
                length: 0,
                byte: data[i],
            });
            i += 1;
        } else {
            compressed.push(Encoded {
                offset,
                length,
                byte: 0,
            });
            i += length;
        }
    }

    compressed
}

fn lz77_decode(compressed: &[Encoded]) -> Vec<u8> {
    let mut decompressed = Vec::new();

    for enc in compressed.iter() {
        if enc.length == 0 {
            decompressed.push(enc.byte);
        } else {
            let start = decompressed.len() - enc.offset;
            for i in start..start + enc.length {
                decompressed.push(decompressed[i]);
            }
        }
    }

    decompressed
}

fn zip_compress(data: &[u8]) -> (VecDeque<bool>, HashMap<char, VecDeque<bool>>) {
    let lz77_encoded = lz77_encode(data);

    let mut lz77_string = String::new();
    for encoded in &lz77_encoded {
        lz77_string.push_str(&format!(
            "{:02}{:02}{}",
            encoded.offset, encoded.length, encoded.byte as char
        ));
    }
    println!("lz77_string: {}", lz77_string);

    // frequency for huffman tree
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for ch in lz77_string.chars() {
        let counter = frequencies.entry(ch).or_insert(0);
        *counter += 1;
    }

    // huffman tree
    let tree = build_tree(&frequencies);

    // codebook
    let mut codebook = HashMap::new();
    build_codebook(&tree, VecDeque::new(), &mut codebook);

    // huffman encode
    let mut huffman_encoded = VecDeque::new();
    for ch in lz77_string.chars() {
        let code = codebook.get(&ch).expect("Character not in codebook");
        huffman_encoded.extend(code.clone());
    }
    println!("codebook: {:?}", codebook);

    (huffman_encoded, codebook)
}

fn zip_decompress(data: &VecDeque<bool>, codebook: &HashMap<char, VecDeque<bool>>) -> Vec<u8> {
    // huffman decode
    let lz77_string = huffman_decode(data, codebook);
    println!("lz77_string: {}", lz77_string);

    // lz77 decode
    let mut lz77_encoded = Vec::new();
    let mut chars = lz77_string.chars();
    while let Some(offset_char) = chars.next() {
        let offset = format!(
            "{}{}",
            offset_char,
            chars.next().expect("Expected another character for offset")
        )
        .parse::<usize>()
        .expect("Failed to parse offset");
        let length = format!(
            "{}{}",
            chars.next().expect("Expected a character for length"),
            chars.next().expect("Expected another character for length")
        )
        .parse::<usize>()
        .expect("Failed to parse length");
        let current_byte = chars.next().expect("Expected a character for current_byte") as u8;
        println!(
            "offset: {}, length: {}, current_byte: {}",
            offset, length, current_byte
        );

        lz77_encoded.push(Encoded {
            offset,
            length,
            byte: current_byte,
        });
    }

    lz77_decode(&lz77_encoded)
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
    let input = "ABRACADABRACADABRA".as_bytes().to_vec();
    let (compressed, codebook) = zip_compress(&input);
    let decompressed = zip_decompress(&compressed, &codebook);
    assert_eq!(input, decompressed);

    println!("Original: {:?}", input);
    println!("Compressed: {:?}", compressed);
    println!("Decompressed: {:?}", decompressed);
    println!("Original size: {} bits", input.len() * 8);
    println!("Compressed size: {} bits", compressed.len());
}
