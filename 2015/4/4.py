import hashlib

def parse_file(filename):
    with open(filename, 'r') as file:
        return file.read().strip()

def advent_key_5(input):
    i = 1
    found = False
    while not found:
        s = input + str(i)
        md5_hash = hashlib.md5(s.encode()).hexdigest()
        if md5_hash[:5] == "00000":
            found = True
        i += 1
    return i - 1

def advent_key_6(input):
    i = 1
    found = False
    while not found:
        s = input + str(i)
        md5_hash = hashlib.md5(s.encode()).hexdigest()
        if md5_hash[:6] == "000000":
            found = True
        i += 1
    return i - 1

filename = "4.txt"
input = parse_file(filename)
ans1 = advent_key_5(input)
ans2 = advent_key_6(input)
print("Answer for part 1: ", ans1)
print("Answer for part 2:", ans2)
