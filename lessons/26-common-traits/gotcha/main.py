from dataclasses import dataclass


@dataclass(frozen=True)
class Version:
    major: int
    minor: int


notes = {}
notes[Version(1, 2)] = "stable"
print("found:", notes.get(Version(1, 2)))
