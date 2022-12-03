from itertools import zip_longest

def grouper(iterable, n):
    args = [iter(iterable)] * n
    return zip_longest(*args, fillvalue=None)

f = open("../data/in/3.in", "r")

sum = 0

# HashMap a-z and A-Z
map = dict()

for i in range(26):
    map[chr(i + 97)] = i + 1
    map[chr(i + 65)] = i + 27

count_map_1 = dict()
count_map_2 = dict()

for (l1, l2, l3) in grouper(f.read().splitlines(), 3):
   # Count first
    for ch in l1:
        count_map_1[ch] = count_map_1.get(ch, 0) + 1
    
    # Count second
    for ch in l2:
        count_map_2[ch] = count_map_2.get(ch, 0) + 1
    
    # Find in third the one that is first and second
    for ch in l3:
        if count_map_1.get(ch, 0) > 0 and count_map_2.get(ch, 0) > 0:
            sum += map[ch]
            break

    count_map_1.clear()
    count_map_2.clear()
    
print(sum)