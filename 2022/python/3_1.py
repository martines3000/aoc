f = open("../data/in/3.in", "r")

sum = 0

# HashMap a-z and A-Z
map = dict()

for i in range(26):
    map[chr(i + 97)] = i + 1
    map[chr(i + 65)] = i + 27

count_map = dict()

for line in f.read().splitlines():
    # Split line in half
    left = line[:len(line)//2]
    right = line[len(line)//2:]

    # Count letters in left
    for ch in left:
        count_map[ch] = count_map.get(ch, 0) + 1
    
    # Find the one which is in the left half
    for ch in right:
        if count_map.get(ch, 0) > 0:
            sum += map[ch]
            break

    count_map.clear()
 

print(sum)