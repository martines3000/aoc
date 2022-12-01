f = open("../data/in/1.in", "r")

sum = 0
best = 0

for line in f.read().splitlines():
    if line == "":
        best = max(sum, best)
        sum = 0
    else:
        sum += int(line)

print(best)