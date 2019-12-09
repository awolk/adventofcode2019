fn part1(image: &str) {
    let mut layers: Vec<Vec<u8>> = Vec::new();
    let mut input = image;
    while !input.is_empty() {
        let (layer, rest) = input.split_at(25 * 6);
        layers.push(layer.bytes().map(|c| c - b'0').collect());
        input = rest;
    }

    let layer = layers
        .iter()
        .min_by_key(|layer| bytecount::count(layer, 0))
        .expect("no layers");

    let ones = bytecount::count(layer, 1);
    let twos = bytecount::count(layer, 2);
    let res = ones * twos;

    println!("Part 1: {}", res);
}

fn main() {
    let image = include_str!("input.txt");
    part1(image);
}
