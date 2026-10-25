print('"a,b,".split(","):', "a,b,".split(","))
print('"".split(","):', "".split(","))
print('" a  b\\t".split():', " a  b\t".split())
print('"".split():', "".split())
print('"k,v,w".split(",", 1):', "k,v,w".split(",", 1))

for text in ["42", "+42", " 42", "1_000", "٣", "42abc"]:
    try:
        print(f"int({text!r}):", int(text))
    except ValueError as e:
        print(f"int({text!r}): ValueError:", e)

print('"a\\x1cb".split():', "a\x1cb".split())
print('"xxabxx".lstrip("xa"):', "xxabxx".lstrip("xa"))
print('"a-a-a".replace("-", "+", 1):', "a-a-a".replace("-", "+", 1))
print('"a\\rb\\r\\nc\\n\\x0bd".splitlines():', "a\rb\r\nc\n\x0bd".splitlines())
print('int("9" * 30):', int("9" * 30))
print('"straße".upper():', "straße".upper())
print('"É".lower() == "é":', "É".lower() == "é")
print('"STRASSE".casefold() == "straße".casefold():', "STRASSE".casefold() == "straße".casefold())
