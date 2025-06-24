

def parse_file(filename):
    text = ""
    with open(filename, 'r') as file:
        for line in file:
            text = line.strip()
    return text

def floor_counter(text):
    n = len(text)
    floor = 0
    for i in range(n):
        if "(" == text[i]:
            floor += 1
        else:
            floor -= 1
    return floor
    
def first_chance(text):
    n = len(text)
    floor = 0
    for i in range(n):
        if "(" == text[i]:
            floor += 1
        else:
            floor -= 1
            if floor == -1:
                return i
    return -1


text = parse_file("1.txt")
ans1 = floor_counter(text)
ans2 = first_chance(text) + 1 # This is not considered as a 0 index array
print("Answer for part 1: ", ans1)
print("Answer for part 2: ", ans2)