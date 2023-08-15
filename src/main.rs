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

fn main() {
    let input = "ABRACADABRAABRACADABRA".as_bytes().to_vec();
    let compressed = lz77_encode(&input);
    let decompressed = lz77_decode(&compressed);
    assert_eq!(input, decompressed);

    println!("Original: {:?}", input);
    println!("Compressed: {:?}", compressed);
    println!("Decompressed: {:?}", decompressed);
    println!("Original length: {}", input.len());
    println!("Compressed length: {}", compressed.len());
}
