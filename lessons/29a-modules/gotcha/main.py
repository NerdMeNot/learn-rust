def _discount_percent(qty: int) -> int:
    return 20 if qty >= 10 else 0


def total(qty: int) -> int:
    full = qty * 10
    return full - full * _discount_percent(qty) // 100


print("total:", total(12))
print(f"discount: {_discount_percent(12)}%")
