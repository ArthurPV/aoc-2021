def part_1(arr):
    h = 0
    d = 0
    for i, s in enumerate(arr):
        if s == 'down':
            d += int(arr[i+1])
        elif s == 'forward':
            h += int(arr[i+1])
        elif s == 'up':
            d -= int(arr[i+1])
        else:
            AssertionError('error')
        i += 1
    return h*d

def part_2(arr):
    h = 0
    d = 0
    a = 0
    for i, s in enumerate(arr):
        if s == 'down':
            a += int(arr[i+1])
        elif s == 'forward':
            h += int(arr[i+1])
            d += int(arr[i+1]) * a
        elif s == 'up':
            a -= int(arr[i+1])
        else:
            AssertionError('error')
        i += 1
    return h*d

f = open("input.txt", "r")
a = f.read().split()
print(part_1(a))
print(part_2(a))
