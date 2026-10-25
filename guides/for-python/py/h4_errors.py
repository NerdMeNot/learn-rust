def parse_port(text):
    return int(text)


for text in ["8080", "http"]:
    try:
        print(parse_port(text))
    except ValueError as e:
        print("bad port:", e)
