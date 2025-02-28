def parsing(file_path):
    with open(file_path, 'r') as f:
        return f.read().strip()

def mapping(diskmap):
    res = []
    index = 0
    spaces = []
    contains = {}
    for i in range(len(diskmap)):
        n = int(diskmap[i])
        if i % 2 == 0:
            for j in range(n):
                res.append(index)
            contains[index] = [n, len(res)-n, len(res)-1]
            index += 1
        else:
            for j in range(n):
                res.append(-1)
            spaces.append([n, len(res)-n, len(res)-1])
    return res, contains, spaces, index  


def swapping(res, contains, spaces, index):
    for i in range(index - 1, -1, -1):
        for k in range(len(spaces)):
            space_size, space_start, space_end = spaces[k]

            if contains[i][0] <= space_size and contains[i][1] > space_start:
                for j in range(space_start, space_start + contains[i][0]):
                    res[j] = i
                
                remaining_space = space_size - contains[i][0]
                if remaining_space > 0:
                    spaces[k] = [remaining_space, space_start + contains[i][0], space_end]
                else:
                    spaces.pop(k)

                for j in range(contains[i][1], contains[i][2] + 1):
                    res[j] = -1
                
                break
    return res


def calculate_checksum(res):
    checksum = 0
    for i in range(len(res)):
        if res[i] != -1:
            checksum += i * res[i]
    return checksum

if __name__ == "__main__":
    # diskmap = "2333133121414131402"
    diskmap = parsing("/home/minindu/AoC/2024/AoC2024_9/AoC2024_9.txt")
    res, contains, spaces, index = mapping(diskmap)
    res = swapping(res, contains, spaces, index)
    checksum = calculate_checksum(res)
    print(checksum)