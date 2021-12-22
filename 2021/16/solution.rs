use std::fs;

struct Packet {
    len: usize,
    version_sum: u32,
    value: u64,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> String {
    fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect()
}

fn part1(input: &str) -> u32 {
    parse_packet(input).version_sum
}

fn part2(input: &str) -> u64 {
    parse_packet(input).value
}

fn parse_packet(transmission: &str) -> Packet {
    let mut cursor = 0;
    let mut version_sum = u32::from_str_radix(&transmission[cursor..(cursor + 3)], 2).unwrap();
    cursor += 3;
    let type_id = u8::from_str_radix(&transmission[cursor..(cursor + 3)], 2).unwrap();
    cursor += 3;
    if type_id == 4 {
        let mut value: u64 = 0;
        loop {
            let continue_parsing = &transmission[cursor..(cursor + 1)] == "1";
            cursor += 1;
            value = value << 4;
            value += u64::from_str_radix(&transmission[cursor..(cursor + 4)], 2).unwrap();
            cursor += 4;
            if !continue_parsing {
                return Packet {
                    len: cursor,
                    version_sum,
                    value,
                };
            }
        }
    }
    let length_type_id = &transmission[cursor..(cursor + 1)];
    cursor += 1;
    let mut sub_packet_values = vec![];
    match length_type_id {
        "0" => {
            let sub_packets_length =
                usize::from_str_radix(&transmission[cursor..(cursor + 15)], 2).unwrap();
            cursor += 15;
            let sub_packets_end_index = cursor + sub_packets_length;
            while cursor < sub_packets_end_index {
                let sub_packet = parse_packet(&transmission[cursor..sub_packets_end_index]);
                cursor += sub_packet.len;
                version_sum += sub_packet.version_sum;
                sub_packet_values.push(sub_packet.value);
            }
        }
        "1" => {
            let sub_packets_count =
                usize::from_str_radix(&transmission[cursor..(cursor + 11)], 2).unwrap();
            cursor += 11;
            for _ in 0..sub_packets_count {
                let sub_packet = parse_packet(&transmission[cursor..]);
                cursor += sub_packet.len;
                version_sum += sub_packet.version_sum;
                sub_packet_values.push(sub_packet.value);
            }
        }
        _ => unreachable!(),
    }
    let value = match type_id {
        0 => sub_packet_values.into_iter().sum(),
        1 => sub_packet_values.into_iter().product(),
        2 => sub_packet_values.into_iter().min().unwrap(),
        3 => sub_packet_values.into_iter().max().unwrap(),
        5 => (sub_packet_values[0] > sub_packet_values[1]) as u64,
        6 => (sub_packet_values[0] < sub_packet_values[1]) as u64,
        7 => (sub_packet_values[0] == sub_packet_values[1]) as u64,
        _ => unreachable!(),
    };
    Packet {
        len: cursor,
        version_sum,
        value,
    }
}
