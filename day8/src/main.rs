struct Image {
    layers: Vec<Vec<u8>>,
}

impl Image {
    fn parse(data: &str) -> Image {
        let mut layers: Vec<Vec<u8>> = Vec::new();
        let mut remaining = data;
        while !remaining.is_empty() {
            let (layer, rest) = remaining.split_at(25 * 6);
            layers.push(layer.bytes().map(|c| c - b'0').collect());
            remaining = rest;
        }

        Image { layers }
    }

    fn decode(&self) -> [u8; 25 * 6] {
        let mut res = [2; 25 * 6];

        for layer in &self.layers {
            for i in 0..(25 * 6) {
                if res[i] == 2 {
                    res[i] = layer[i]
                }
            }
        }

        res
    }
}

fn part1(img_data: &str) {
    let image = Image::parse(img_data);

    let layer = image
        .layers
        .iter()
        .min_by_key(|layer| bytecount::count(layer, 0))
        .expect("no layers");

    let ones = bytecount::count(layer, 1);
    let twos = bytecount::count(layer, 2);
    let res = ones * twos;

    println!("Part 1: {}", res);
}

fn part2(img_data: &str) {
    let image = Image::parse(img_data);
    let decoded = image.decode();
    println!("Part 2:");
    for row in 0..6 {
        for column in 0..25 {
            print!(
                "{}",
                match decoded[row * 25 + column] {
                    0 => " ",
                    1 => "#",
                    _ => panic!("invalid pixel"),
                }
            )
        }
        println!()
    }
}

fn main() {
    let image_data = include_str!("input.txt");
    part1(image_data);
    part2(image_data);
}
