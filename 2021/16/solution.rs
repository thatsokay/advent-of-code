use std::fs;

struct Packet {
    version_sum: u64,
    value: u64,
}

struct Bits<'a> {
    message: &'a [u8],
    cursor: usize,
}

impl<'a> Bits<'a> {
    fn new(message: &'a [u8]) -> Self {
        Self { message, cursor: 0 }
    }

    fn read(&mut self, length: usize) -> u64 {
        let start_byte_index = self.cursor / 8;
        let start_bit_index = self.cursor % 8;
        let end_byte_index = (self.cursor + length) / 8;
        let end_bit_index = (self.cursor + length) % 8;

        let mut value;
        if start_byte_index < end_byte_index {
            value = (self.message[start_byte_index] << start_bit_index >> start_bit_index) as u64;

            for &byte in &self.message[(start_byte_index + 1)..(end_byte_index)] {
                value <<= 8;
                value |= byte as u64;
            }

            if end_bit_index > 0 {
                value <<= end_bit_index;
                value |= (self.message[end_byte_index] >> (8 - end_bit_index)) as u64;
            }
        } else {
            value = (self.message[start_byte_index] << start_bit_index
                >> (start_bit_index + 8 - end_bit_index)) as u64
        }

        self.cursor += length;
        value
    }

    fn parse_literal(&mut self) -> u64 {
        let mut value = 0;
        loop {
            let last_group = self.read(1) == 0;
            value <<= 4;
            value |= self.read(4);
            if last_group {
                return value;
            }
        }
    }
}

fn parse_packet(bits: &mut Bits) -> Packet {
    let mut version_sum = bits.read(3);
    let type_id = bits.read(3);

    if type_id == 4 {
        return Packet {
            version_sum,
            value: bits.parse_literal(),
        };
    }

    let mut operands = vec![];
    let length_type_id = bits.read(1);
    if length_type_id == 0 {
        let sub_packets_length = bits.read(15) as usize;
        let cursor_end_index = bits.cursor + sub_packets_length;
        while bits.cursor < cursor_end_index {
            let sub_packet = parse_packet(bits);
            version_sum += sub_packet.version_sum;
            operands.push(sub_packet.value);
        }
    } else {
        let sub_packets_count = bits.read(11);
        for _ in 0..sub_packets_count {
            let sub_packet = parse_packet(bits);
            version_sum += sub_packet.version_sum;
            operands.push(sub_packet.value);
        }
    }

    let value = match type_id {
        0 => operands.into_iter().sum(),
        1 => operands.into_iter().product(),
        2 => operands.into_iter().min().unwrap(),
        3 => operands.into_iter().max().unwrap(),
        5 => (operands[0] > operands[1]) as u64,
        6 => (operands[0] < operands[1]) as u64,
        7 => (operands[0] == operands[1]) as u64,
        _ => unreachable!(),
    };
    Packet { version_sum, value }
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<u8> {
    fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            (((chunk[0] as char).to_digit(16).unwrap() << 4)
                | ((chunk[1] as char).to_digit(16).unwrap())) as u8
        })
        .collect()
}

fn part1(input: &[u8]) -> u64 {
    parse_packet(&mut Bits::new(input)).version_sum
}

fn part2(input: &[u8]) -> u64 {
    parse_packet(&mut Bits::new(input)).value
}
