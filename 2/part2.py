def common_chars(str1, str2):
    return ''.join([x for x, y in zip(str1, str2) if x is y])

def find_boxes(box_ids):
    num_boxes = len(box_ids)
    for i in range(num_boxes):
        for j in range(i + 1, num_boxes):
            common = common_chars(box_ids[i], box_ids[j])
            if len(common) == 26: # Length of id + newline - 1
                return common
    return None

if __name__ == '__main__':
    with open('input.txt') as f:
        print(find_boxes(f.readlines()))
