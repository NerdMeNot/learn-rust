from dataclasses import dataclass, replace


@dataclass
class Config:
    name: str
    port: int
    verbose: bool


base = Config("service", 8080, False)
test = replace(base, port=9000)
print(base)
print(test)
