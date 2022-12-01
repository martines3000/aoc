from queue import PriorityQueue

f = open("../data/in/1.in", "r")

curr_sum = 0
hq = PriorityQueue()

for line in f.read().splitlines():
    if line == "":
        hq.put(-curr_sum)
        curr_sum = 0
    else:
        curr_sum += int(line)

print(sum([-hq.get() for _ in range(3)])) 