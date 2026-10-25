try:
    try:
        int("eighty")
    except ValueError as e:
        raise RuntimeError("reading bad.conf") from e
except RuntimeError as e:
    print(f"{e}: {e.__cause__}")
