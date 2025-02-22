# diskmap = "2333133121414131402"


def parsing(file_path):
    with open(file_path, 'r') as f:
        return f.read().strip()

def mapping(diskmap):
    res = []
    index = 0
    for i in range(len(diskmap)):
        if i % 2 == 0:
            n = int(diskmap[i])
            for j in range(n):
                res.append(index)
            index += 1
        else:
            n = int(diskmap[i])
            for j in range(n):
                res.append(-1)
    return res

def swapping(res):
    left = 0
    right = len(res) - 1
    while left < right:
        while left < right and res[left] != -1:
            left += 1
        while left < right and res[right] == -1:
            right -= 1
        if left < right:
            res[left], res[right] = res[right], res[left]
            left += 1
            right -= 1
    return res        

def calculate_checksum(res):
    checksum = 0
    i = 0
    while(res[i] != -1):
        checksum += i * res[i]
        i += 1
    return checksum

if __name__ == "__main__":
    diskmap = parsing("AoC2024_9.txt")
    res = mapping(diskmap)
    res = swapping(res)
    checksum = calculate_checksum(res)
    print(checksum)