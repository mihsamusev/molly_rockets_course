import json
import math
import time

EARTH_RADIUS_KM = 6371


def haversine(x0, y0, x1, y1, r):
    dy = math.radians(y1 - y0)
    dx = math.radians(x1 - x0)
    y0 = math.radians(y0)
    y1 = math.radians(y1)

    root = (math.sin(dy / 2) ** 2) + math.cos(y0) * math.cos(y1) * (
        math.sin(dx / 2) ** 2
    )
    result = 2 * r * math.asin(math.sqrt(root))

    return result


def load_pairs(filename):
    with open(filename, "r") as fin:
        pairs = json.load(fin)
    return pairs


def main():
    input_start = time.time()
    pairs = load_pairs("pairs.json")
    n_pairs = len(pairs)
    input_time = time.time() - input_start

    math_start = time.time()
    sum = 0
    for pair in pairs:
        sum += haversine(
            pair["x0"], pair["y0"], pair["x1"], pair["y1"], EARTH_RADIUS_KM
        )
    average = sum / n_pairs

    math_time = time.time() - math_start
    throughput = n_pairs / math_time
    print(f"{average=} km")
    print(f"{input_time=} s")
    print(f"{math_time=} s")
    print(f"{throughput=} distances per second")


if __name__ == "__main__":
    main()
