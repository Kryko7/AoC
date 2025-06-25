
def parse_file(filename):
    input = []
    with open(filename, 'r') as file:
        for line in file:
            parts = line.strip().split('x')
            input.append(list(map(int, parts)))
    return input

def wrapping_papers(input):
    total =0
    for dims in input:
        wrapping = 2*dims[0]*dims[1] + 2*dims[0]*dims[2] + 2*dims[1]*dims[2]
        dims.sort()
        slack = dims[0]*dims[1]
        total += wrapping + slack
    return total

def ribbons(input):
    total = 0
    for dims in input:
        wrapping = dims[0]*dims[1]*dims[2]
        dims.sort()
        slack = 2*dims[0] + 2*dims[1]
        total += wrapping + slack
    return total


filename = "2.txt"
input = parse_file(filename)
ans1 = wrapping_papers(input)
ans2 = ribbons(input)
print("Answer for the part1: ", ans1)
print("Answer for the part2: ", ans2)
    