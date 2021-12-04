def part_1(a):
    gama = ''
    espion = ''
    for i in range(0, 12):
        n_0 = list(map(lambda v: v[i] == '0', a)).count(True)
        if n_0 > len(a) / 2:
            gama += '0'
            espion += '1'
        else:
            gama += '1'
            espion += '0'
    return int(gama, 2) * int(espion, 2)

def keep_value(idx_l, s_l):
    k_v = []
    for v in idx_l:
        k_v.append(s_l[v])
    return k_v

def get_index(i, s_l):
    n_0 = []
    n_1 = []
    for n, s in enumerate(s_l):
        if s[i] == '0':
            n_0.append(n)
        else:
            n_1.append(n)
    return n_0, n_1

def part_2(a):
    gen = a
    co2 = a

    for i in range(0, 12):
        if len(gen) > 1:
            n_0 = get_index(i, gen)[0]
            n_1 = get_index(i, gen)[1]
            if len(n_0) > len(n_1):
                gen = keep_value(n_0, gen)
            elif len(n_0) <= len(n_1):
                gen = keep_value(n_1, gen)

        if len(co2) > 1:
            n_0 = get_index(i, co2)[0]
            n_1 = get_index(i, co2)[1]
            if len(n_0) <= len(n_1):
                co2 = keep_value(n_0, co2)
            elif len(n_0) > len(n_1):
                co2 = keep_value(n_1, co2)
    return int(gen[0], 2) * int(co2[0], 2)

f = open("input.txt", "r")
a = f.read().split()
print(part_1(a))
print(part_2(a))
