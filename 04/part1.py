from collections import Counter, defaultdict

def most_sleep(records):
    index = 0
    total_time_slept = defaultdict(int)
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
            time_slept = end_sleep - start_sleep
            total_time_slept[guard_id] += time_slept
    most_sleep_id = max(total_time_slept.items(), key=(lambda x: x[1]))[0]
    return most_sleep_id, slept_minutes[most_sleep_id].most_common()[0][0]

if __name__ == '__main__':
    with open('input.txt') as f:
        records = f.readlines()
        records.sort()
        guard_id, minute = most_sleep(records)
        print(guard_id * minute)
