import re


def parse_input():
    with open('input.txt') as f:
        contents = f.read()
    return [parse_passport(passport) for passport in contents.split('\n\n')]

required_fields = {
    'byr': lambda val: 1920 <= int(val) <= 2002,
    'iyr': lambda val: 2010 <= int(val) <= 2020,
    'eyr': lambda val: 2020 <= int(val) <= 2030,
    'hgt': lambda val: bool(re.compile(r'^\d+(cm|in)$').match(val)) and
        (150 <= int(val[:-2]) <= 193
        if val[-2:] == 'cm'
        else 59 <= int(val[:-2]) <= 76),
    'hcl': lambda val: bool(re.compile(r'^#[0-9a-f]{6}$').match(val)),
    'ecl': lambda val: val in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'),
    'pid': lambda val: bool(re.compile(r'^\d{9}$').match(val)),
}

def part1(inputs):
    valid_count = 0
    for passport in inputs:
        for field in required_fields.keys():
            if field not in passport:
                break
        else:
            valid_count += 1
    return valid_count

def part2(inputs):
    valid_count = 0
    for passport in inputs:
        for field, validator in required_fields.items():
            if field not in passport:
                break
            if not validator(passport[field]):
                break
        else:
            valid_count += 1
    return valid_count

def parse_passport(passport):
    split_fields = (field.split(':') for field in passport.split())
    return {key: val for key, val in split_fields}


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
