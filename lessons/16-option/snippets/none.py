ages = {"ana": 31, "zed": None}

# dict.get gives None for a missing key, and for a key that holds None.
print("ana:", ages.get("ana"))
print("zed (holds None):", ages.get("zed"))
print("bo (missing):", ages.get("bo"))
print("zed in ages:", "zed" in ages)

# Nothing stops you using the result as if it were there.
try:
    ages.get("bo").bit_length()
except AttributeError as e:
    print("AttributeError:", e)
