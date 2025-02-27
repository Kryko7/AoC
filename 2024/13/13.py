def determinant(a1, a2, b1, b2):
    return a1 * b2 - a2 * b1

def solve_equations(a1, a2, b1, b2, c1, c2):
    d = determinant(a1, a2, b1, b2)
    
    # If determinant is zero, there is no unique solution
    if d == 0:
        return diophantine_solutions(a1, b1, c1)
    
    # Compute the numerators for x and y using Cramer's Rule
    x_numerator = c1 * b2 - c2 * b1
    y_numerator = a1 * c2 - a2 * c1
    
    # Compute x and y
    x = x_numerator / d
    y = y_numerator / d
    
    # Check if x and y are integers
    if x.is_integer() and y.is_integer():
        return [[int(x), int(y)]]
    else:
        return []
    

# This part is needed to study. Used a maths algorithm but this part is actually not needed since
# the condition of d == 0 does not fullfilled in this particular problem.
def diophantine_solutions(a, b, c):
    from math import gcd
    
    g = gcd(a, b)
    
    if c % g != 0:
        return []  # No integer solutions exist
    
    # Scale down the equation
    a, b, c = a // g, b // g, c // g

    # Find a particular solution using Extended Euclidean Algorithm
    def extended_gcd(a, b):
        if b == 0:
            return (1, 0)
        x1, y1 = extended_gcd(b, a % b)
        return (y1, x1 - (a // b) * y1)

    x0, y0 = extended_gcd(a, b)
    
    x0 *= c
    y0 *= c

    # General solution: x = x0 + k * (b / g), y = y0 - k * (a / g)
    solutions = []
    for k in range(-10, 10):  # Arbitrary range for checking integer solutions
        x = x0 + k * (b)
        y = y0 - k * (a)
        if x > 0 and y > 0:  # Ensure positive integer solutions
            solutions.append((x, y))
    
    return solutions

import re


def parsing(file_path):
    with open(file_path, 'r') as f:
        partions =  f.read().strip().split('\n')
        n = len(partions)
        equations = []
        pattern1 = r"Button A:\s*X\+(\d+),\s*Y\+(\d+)"
        pattern2 = r"Button B:\s*X\+(\d+),\s*Y\+(\d+)"
        pattern3 = r"Prize: X=(\d+), Y=(\d+)"
        equation = []
        for i in range(n):
            text = partions[i]
            if i % 4 == 0:
                match = re.search(pattern1, text)
                if match:
                    a1 = int(match.group(1))
                    a2 = int(match.group(2))
                    equation.append(a1)
                    equation.append(a2)
            if i % 4 == 1:
                match = re.search(pattern2, text)
                if match:
                    b1 = int(match.group(1))
                    b2 = int(match.group(2))
                    equation.append(b1)
                    equation.append(b2)
            if i % 4 == 2:
                match = re.search(pattern3, text)
                if match:
                    c1 = int(match.group(1))
                    c2 = int(match.group(2))
                    equation.append(c1)
                    equation.append(c2)
                    equations.append(equation[:])
                    equation = []
        return equations


                

# result = solve_equations(a1, a2, b1, b2, c1, c2)
# print(result)

equations = parsing("/home/ishvalin/personal/AoC/2024/13/13.txt")
results = []
tokens = 0
mul_sols = 0
for equation in equations:
    a1 = equation[0]
    a2 = equation[1]
    b1 = equation[2]
    b2 = equation[3]
    c1 = equation[4] + 10000000000000
    c2 = equation[5] + 10000000000000
    result = solve_equations(a1, a2, b1, b2, c1, c2)
    if(len(result) > 0):
        result.sort()
        tokens += 3 * result[0][0] + result[0][1]
        # results.append(result)
    
    if(len(result) > 1):
        mul_sols += 1

# print(results)
print(tokens)
print(mul_sols)