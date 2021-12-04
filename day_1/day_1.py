def part_1(a):
    n = 0
    for i, v in enumerate(a):
        if i < len(a) - 1:
            if v < a[i+1]:
                n += 1
    return n

def part_2(a):
    n = 0
    for i, v in enumerate(a):
        if i < len(a) - 3:
            if v + a[i+1] + a[i+2] < a[i+1] + a[i+2] + a[i+3]:
                n += 1
    return n


f = open("input.txt", "r")
a = list(map(lambda x: int(x), f.read().split()))
print(part_1(a))
print(part_2(a))
