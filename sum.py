def sum_range(data):
    total = 0
    for i in range(len(data)):
        total += data[i]
    return total


def sum_builtin(data):
    return sum(data)
