def get_boards(lines):
    numbers = list(map(int, lines.pop(0).split(',')))
    boards = []
    c_board = []
    while len(lines) > 0:
        prev = lines.pop(0)
        if not prev and len(c_board) > 0:
            boards.append(c_board)
            c_board = []
        elif prev:
            transform_line = list(map(int, list(filter(lambda x: x != '', prev.split(' ')))))
            c_board.append([{ 'v': x, 'm': False } for x in transform_line])
    if len(c_board) > 0:
        boards.append(c_board)
    return numbers, boards

def check_board(board):
    for row in board:
        if len(list(filter(lambda x: x['m'] == False, row))) == 0:
            return True
    for i in range(len(board)):
        column = [row[i] for row in board]
        if len(list(filter(lambda x: x['m'] == False, column))) == 0:
            return True
    return False

def mark_item(number, board):
    for row in board:
        for item in row:
            if item['v'] == number:
                item['m'] = True
                return

def calc_sum(board):
    sum = 0
    for row in board:
        for item in row:
            if item['m'] == False:
                sum += item['v']
    return sum

def part_1_and_2(set, boards):
    b_w_sums = []
    w_bs = []
    for n in set:
        for i, board in enumerate(boards):
            if i in w_bs:
                continue
            mark_item(n, board)
            if check_board(board):
                b_w_sums.append(calc_sum(board) * n)
                w_bs.append(i)
                continue
    return b_w_sums

f = open('input.txt')
lines = f.read().splitlines()
numbers, boards = get_boards(lines.copy())
result = part_1_and_2(numbers, boards)
print(result[0])
print(result[-1])