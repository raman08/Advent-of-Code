ROW_MULTIPLIER = 100
COLUMN_MULTIPLIER = 1

with open("./input.txt", "r") as file:
    grids = file.read().strip().split("\n\n")


def find_reflected_rows(grid, smudges=0):
    for r in range(1, len(grid)):
        above = grid[:r][::-1]
        below = grid[r:]

        if (
            sum(
                sum(0 if a == b else 1 for a, b in zip(x, y))
                for x, y in zip(above, below)
            )
            == smudges
        ):
            return r

    return 0


solution1 = 0
solution2 = 0
for grid in grids:
    block = grid.splitlines()
    solution1 += find_reflected_rows(block) * ROW_MULTIPLIER
    solution2 += find_reflected_rows(block, smudges=1) * ROW_MULTIPLIER

    # transpose
    block = list(zip(*block))

    solution1 += find_reflected_rows(block) * COLUMN_MULTIPLIER
    solution2 += find_reflected_rows(block, smudges=1) * COLUMN_MULTIPLIER

print("Solution 1:", solution1)
print("Solution 2:", solution2)
