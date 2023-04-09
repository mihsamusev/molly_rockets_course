import json
import sys

import numpy as np

N_POINTS = 10
if len(sys.argv) == 2:
    N_POINTS = int(sys.argv[-1])

keys = ("x0", "y0", "x1", "y1")
ranges = np.array([180, 360, 180, 360])

print(f"Generating {N_POINTS} lat/lon pairs")
coefficients = np.random.rand(N_POINTS, 4)
rows = (coefficients - 0.5) * ranges


pairs = [dict(zip(keys, row)) for row in rows]

filename = "pairs.json"
print(f"saving to {filename=}")
with open(filename, "w") as fout:
    json.dump(pairs, fout)
