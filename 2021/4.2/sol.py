import numpy as np

input_file = open("in.txt", "r")

draw_nums = list(map(int, input_file.readline().split(",")))
board_nums = []

for line in input_file.readlines():
    if(line.strip() != ''):
        board_nums.extend(list(map(int, line.strip().split())))

boards = np.array(board_nums).reshape(len(board_nums)//25, 5, 5)
hits = np.zeros((len(board_nums)//25, 5, 5), np.bool_)

remaining = np.zeros((boards.shape[0], 1), np.bool_)

for num in draw_nums:
    boards = boards[np.where(remaining == False)[0]]
    hits = hits[np.where(remaining == False)[0]]
    remaining = remaining[np.where(remaining == False)[0]]
    hits[np.where(boards == num)] = True

    cols = np.where(np.all(hits, axis=1) == True)[0]
    rows = np.where(np.all(hits, axis=2) == True)[0]
    if(cols.size > 0):
        if(remaining.size == 1):
            sum = np.sum(np.array(np.where(hits == False, boards, 0)))
            print(sum * num)

        remaining[cols] = True

    if(rows.size > 0):
        if(remaining.size == 1):
            sum = np.sum(np.array(np.where(hits == False, boards, 0)))
            print(sum * num)

        remaining[rows] = True
