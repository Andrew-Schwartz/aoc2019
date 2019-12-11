type Layer = Vec<char>; // len == 150

#[aoc_generator(day8)]
fn gen(input: &str) -> Vec<Layer> {
    input.chars()
        .collect::<Vec<_>>()
        .chunks(150)
        .map(|slice| slice.to_vec())
        .collect()
}

#[aoc(day8, part1)]
fn part1(layers: &Vec<Layer>) -> usize {
    let layer = layers.iter()
        .min_by_key(|layer| layer.iter()
            .filter(|&&pixel| pixel == '0')
            .count())
        .unwrap();
    let num1 = layer.iter().filter(|&&px| px == '1').count();
    let num2 = layer.iter().filter(|&&px| px == '2').count();
    num1 * num2
}

#[aoc(day8, part2)]
fn part2(layers: &Vec<Layer>) -> String {
    let mut img = Layer::new();
    for _i in 0..150 {
        img.push('?');
    }

    for layer in layers {
        let new_data: Vec<(usize, char)> = layer.iter()
            .enumerate()
            .filter(|(_, &px)| px != '2')
            .filter(|(i, _)| img[*i] == '?')
            .map(|(i, &px)| (i, if px == '0' { ' ' } else { '█' }))
            .collect();
        for (i, px) in new_data {
            img[i] = px;
        }
    }

    let mut string = String::from("\n");
    for y in 0..6 {
        for x in 0..25 {
            string.push(img[x + 25 * y]);
        }
        string.push('\n');
    }
    string
}