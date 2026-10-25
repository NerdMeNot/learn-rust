PRICE = 10


# The leading underscore is a convention: "internal, please don't call".
def _discount_percent(qty: int) -> int:
    return 20 if qty >= 10 else 0


def total(qty: int) -> int:
    full = qty * PRICE
    return full - full * _discount_percent(qty) // 100


print("3 items:", total(3))
print("12 items:", total(12))
