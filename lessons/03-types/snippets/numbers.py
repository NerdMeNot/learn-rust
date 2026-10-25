from decimal import Decimal

print("2 ** 100:", 2**100)
print("7 / 2:", 7 / 2)
print("7 // 2:", 7 // 2)
print("-7 // 2:", -7 // 2)
print("-7 % 2:", -7 % 2)
print("int(-3.9):", int(-3.9))
print("True + 1:", True + 1)
print("nan == nan:", float("nan") == float("nan"))
try:
    print(1.0 / 0.0)
except ZeroDivisionError as e:
    print("1.0 / 0.0:", e)
print('int(" 12 "):', int(" 12 "))
print("Decimal('0.1') + Decimal('0.2'):", Decimal("0.1") + Decimal("0.2"))
print("ord('€'):", ord("€"), " len('😀'):", len("😀"))
