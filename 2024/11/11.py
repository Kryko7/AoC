# initial_state = [125, 17]

# def parse(filename):
#     with open(filename, 'r') as f:
#         lines = f.readlines()
#     stones = []
#     for line in lines:
#         _ = line.strip().split()
#         for stone in _:
#             stones.append(int(stone))
#     return stones

# def has_zero(stone):
#     if stone == 0:
#         return True
#     return False

# def is_even(stone):
#     i = 0
#     while(stone > 0):
#         i += 1
#         stone //= 10
#     return i % 2 == 0, i

# def partition(stone, size):
#     div = 10 ** (size // 2)
#     return stone // div, stone % div

# def blink(number, initial_state):
#     temp_state = []
#     for i in range(number):
#         for stone in initial_state:
#             if has_zero(stone):
#                 temp_state.append(1)
#             else:
#                 is_even_, length = is_even(stone)
#                 if is_even_:
#                     left, right = partition(stone, length)
#                     temp_state.append(left)
#                     temp_state.append(right)
#                 else:
#                     temp_state.append(stone * 2024)
#         initial_state = temp_state
#     return initial_state


# if __name__ == '__main__':
#     stones = parse("/home/minindu/AoC/2024/11/11.txt")
#     print(stones)
#     # print(blink(5, stones))


stones = [125, 17]

def parse(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()
    stones = []
    for line in lines:
        _ = line.strip().split()
        for stone in _:
            stones.append(int(stone))
    return stones

def has_zero(stone):
    if stone == 0:
        return True
    return False

def is_even(stone):
    i = 0
    while(stone > 0):
        i += 1
        stone //= 10
    return i % 2 == 0, i

def partition(stone, size):
    div = 10 ** (size // 2)
    return stone // div, stone % div

def blink(number, stones):
    state_1 = [0 for _ in range(1000000)]
    state_2 = [0 for _ in range(1000000)]
    size = len(stones)
    index = 0
    for i in range(size):
        state_1[i] = stones[i]
    
    for i in range(number):
        index = 0
        for i in range(size):
            if has_zero(state_1[i]):
                state_2[index] = 1
                index += 1
            else:
                is_even_, length = is_even(state_1[i])
                if is_even_:
                    left, right = partition(state_1[i], length)
                    state_2[index] = left
                    state_2[index + 1] = right
                    index += 2
                else:
                    state_2[index] = state_1[i] * 2024
                    index += 1
            
            state_1, state_2 = state_2, state_1
            size = index
    return index


if __name__ == '__main__':
    stones = stones = [125, 17]
    print(stones)
    print(blink(25, stones))