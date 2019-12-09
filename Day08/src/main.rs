use std::io::Read;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const IMAGE_SIZE: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let digits = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit"))
        .collect::<Vec<_>>();
    let layers = digits.chunks(IMAGE_SIZE).collect::<Vec<_>>();
    let layers_zeros_count = layers
        .iter()
        .map(|layer| layer.iter().filter(|digit| **digit == 0).count())
        .collect::<Vec<_>>();
    let (layer_with_least_zeros, zeros_count) = layers
        .iter()
        .zip(&layers_zeros_count)
        .min_by(|layer1, layer2| layer1.1.cmp(&layer2.1))
        .unwrap();
    let ones_count = layer_with_least_zeros.iter().filter(|d| **d == 1).count();
    let twos_count = layer_with_least_zeros.iter().filter(|d| **d == 2).count();
    println!("{}", ones_count * twos_count);

    let mut final_image = vec![2; IMAGE_SIZE];
    for layer in layers {
        for (pos, pixel) in layer.iter().enumerate() {
            if final_image[pos] == 2 {
                final_image[pos] = *pixel;
            }
        }
    }
    for line in final_image.chunks(IMAGE_WIDTH) {
        for d in line {
            print!(
                "{}",
                match d {
                    0 => ' ',
                    1 => '\u{2588}',
                    _ => '?',
                }
            );
        }
        println!();
    }
}
