from concurrent.futures import ProcessPoolExecutor


def square_sum(bounds):
    lo, hi = bounds
    return sum(i * i for i in range(lo, hi))


if __name__ == "__main__":
    n, parts = 1_000_000, 8
    chunks = [(n * p // parts + 1, n * (p + 1) // parts + 1) for p in range(parts)]
    with ProcessPoolExecutor(max_workers=parts) as pool:
        print(sum(pool.map(square_sum, chunks)))
