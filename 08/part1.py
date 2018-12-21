def sum_meta(nums, pos):
    children, meta = nums[pos:pos + 2]
    if children == 0:
        return sum(nums[pos + 2:pos + 2 + meta]), pos + 2 + meta
    else:
        node_sum = 0
        end_pos = pos + 2
        for i in range(children):
            child_sum, end_pos = sum_meta(nums, end_pos)
            node_sum += child_sum
        return node_sum + sum(nums[end_pos:end_pos + meta]), end_pos + meta

if __name__ == '__main__':
    with open('input.txt') as f:
        nums = list(map(int, f.read().split()))
        print(sum_meta(nums, 0))
