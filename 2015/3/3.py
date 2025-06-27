def parse_file(filename):
    parts = []
    with open(filename, 'r') as file:
        for line in file:
            parts = list(line.strip())
    return parts

def unique_houses(input):
    locations = set()
    x , y = 0, 0
    locations.add((x, y))
    for c in input:
        if c == '>':
            x += 1
        elif c == '<':
            x -= 1
        elif c == '^':
            y += 1
        else:
            y -= 1
        locations.add((x, y))
    return len(locations)

def unique_houses2(input):
    locations = set()
    x, y = 0, 0
    a, b = 0, 0
    locations.add((x, y))
    i = 0
    for c in input:
        if i % 2 == 0:
            if c == '>':
                x += 1
            elif c == '<':
                x -= 1
            elif c == '^':
                y += 1
            else:
                y -= 1
            locations.add((x, y))
        else:
            if c == '>':
                a -= 1
            elif c == '<':
                a += 1
            elif c == '^':
                b -= 1
            else:
                b += 1
            locations.add((a, b))
        i += 1
    return len(locations)


filename = "3.txt"
input = parse_file(filename)
ans1 = unique_houses(input)
ans2 = unique_houses2(input)
print("Answer for the part 1:", ans1)
print("Answer for the part 2:", ans2)
