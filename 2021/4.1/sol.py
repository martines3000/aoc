import numpy as np

input_file = open("in.txt", "r")

draw_nums = list(map(int, input_file.readline().split(",")))
board_nums = []

for line in input_file.readlines():
    if(line.strip() != ''):
        board_nums.extend(list(map(int, line.strip().split())))

boards = np.array(board_nums).reshape(len(board_nums)//25, 5, 5)
hits = np.zeros((len(board_nums)//25, 5, 5), np.bool_)

winner = -1

for num in draw_nums:
    if winner >= 0:
        break

    hits[np.where(boards == num)] = True

    cols = np.array(np.vstack(np.meshgrid(
        np.where(np.all(hits, axis=1) == True))))
    rows = np.array(np.vstack(np.meshgrid(
        np.where(np.all(hits, axis=2) == True))))

    if(cols.size > 0):
        winner = cols[0][0]
    if(rows.size > 0):
        winner = rows[0][0]

    if(winner > 0):
        sum = np.sum(
            np.array(np.where(hits[winner, :, :] == False, boards[winner, :, :], 0)))
        print(sum * num)
