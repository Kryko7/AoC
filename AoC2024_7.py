
def fit_combination(key, index, factors, curr_sum):
    if index == len(factors):
        return curr_sum == key
    return fit_combination(key, index + 1, factors, curr_sum + factors[index]) or fit_combination(key, index + 1, factors, curr_sum * factors[index])


def parse_file(filename):
    total = 0
    try:
        with open(filename, 'r') as file:
            for line in file:
                line = line.strip()
                if not line:
                    continue
                
                key, values = line.split(":")
                numbers = [int(x) for x in values.strip().split()]
                key = int(key)
                if fit_combination(key, 1, numbers, numbers[0]):
                    total += key
    except FileNotFoundError:
        print(f"File {filename} not found")
    except Exception as e:
        print(f"Error: {e}")
    finally:
        return total
                



print(parse_file("AoC2024_7.txt"))
