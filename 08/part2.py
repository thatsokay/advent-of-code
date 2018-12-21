from operator import itemgetter

def sum_meta(nums, pos):
    children, meta = nums[pos:pos + 2]
    if children == 0:
        return sum(nums[pos + 2:pos + 2 + meta]), pos + 2 + meta
    else:
        child_sums = []
        end_pos = pos + 2
        for i in range(children):
            child_sum, end_pos = sum_meta(nums, end_pos)
            child_sums.append(child_sum)
        metadata = nums[end_pos:end_pos + meta]
        metadata = filter(lambda x: x <= children, metadata)
        metadata = map(lambda x: x - 1, metadata)
        ref_sums = itemgetter(*metadata)(child_sums)
        node_sum = ref_sums if type(ref_sums) is int else sum(ref_sums)
        return node_sum, end_pos + meta

if __name__ == '__main__':
    with open('input.txt') as f:
        nums = list(map(int, f.read().split()))
        print(sum_meta(nums, 0))
