f = open("../data/in/2.in", "r")

sum = 0

for line in f.read().splitlines():
   match line:
        case "A X": sum += 4
        case "A Y": sum += 8
        case "A Z": sum += 3
        case "B X": sum += 1
        case "B Y": sum += 5
        case "B Z": sum += 9
        case "C X": sum += 7
        case "C Y": sum += 2
        case "C Z": sum += 6

print(sum)