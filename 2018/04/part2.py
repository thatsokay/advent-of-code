from collections import Counter, defaultdict

def most_frequent_minute(records):
    index = 0
    slept_minutes = defaultdict(Counter)
    for record in records:
        parts = record.split()
        if '#' in record:
            guard_id = int(parts[3][1:])
        elif parts[-1] == 'asleep':
            start_sleep = int(parts[1][3:5])
        elif parts[-1] == 'up':
            end_sleep = int(parts[1][3:5])
            slept_minutes[guard_id] += Counter(range(start_sleep, end_sleep))
    choice = max(slept_minutes.items(), key=(lambda x: x[1].most_common()[0][1]))
    return choice[0], choice[1].most_common()[0][0]

if __name__ == '__main__':
    with open('input.txt') as f:
        records = f.readlines()
        records.sort()
        guard_id, minute = most_frequent_minute(records)
        print(guard_id * minute)
