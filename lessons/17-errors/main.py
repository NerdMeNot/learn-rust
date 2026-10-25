def parse_age(s: str) -> int:
    return int(s)  # raises ValueError on bad input; no ? needed


for text in ["42", "abc"]:
    try:
        print(f"age {parse_age(text)}")
    except ValueError as e:
        print(f"error: {e}")
