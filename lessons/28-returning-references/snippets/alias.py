def first(rows):
    return rows[0]  # the same list, not a copy


rows = [[1], [2]]
f = first(rows)
f.append(99)
print("rows:", rows)
