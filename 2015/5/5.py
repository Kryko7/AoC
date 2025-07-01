import re

def parse_file(filename):
    words = []
    with open(filename, 'r') as file:
        for line in file:
            words.append(line.strip())
    return words

def nice_words1(words):
    count = 0
    for word in words:
        match1 = re.search(r"([a-zA-Z])\1", word)
        match2 = re.findall(r"[aeiou]", word)
        match3 = re.search(r"ab|cd|pq|xy", word)

        if match1 and len(match2) >= 3 and not match3:
            count += 1
    return count

def nice_words2(words):
    count = 0
    for word in words:
        match1 = re.search(r"([a-zA-Z]).\1", word)
        match2 = re.search(r"(..).*\1", word)

        if match1 and match2:
            count += 1
    return count

filename = "5.txt"
words = parse_file(filename)
ans1 = nice_words1(words)
ans2 = nice_words2(words)
print("Answer for part 1: ", ans1)
print("Answer for part 2: ", ans2)