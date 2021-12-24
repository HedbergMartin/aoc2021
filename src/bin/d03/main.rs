const BITS: usize = 12;

fn main() {
    let mut bit_counter = [0; BITS];

    include_str!("input.txt").lines().for_each(|line| {
        
        for (index, c) in line.char_indices() {
            match c {
                '1' => bit_counter[index] += 1,
                '0' => bit_counter[index] -= 1,
                _ => (),
            }            
        }
    });

    let mut gamma_rate = 0;
    let mut index = BITS;
    for bit in bit_counter.iter() {
        index -= 1;
        if *bit > 0 {
            set_bit(&mut gamma_rate, index as u8);
        }
    }

    let epsilon_rate = gamma_rate ^ 0b111111111111; //Inverts 12 first bit. ! does not work since it will flip all 32 bits

    println!("Gamma rate = {}, Epsilon rate = {}, Power consumption = {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}

fn set_bit(value: &mut i32, bit: u8) {
    *value |= 1 << bit;
}
