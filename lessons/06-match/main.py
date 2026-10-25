day = 6
match day:
    case 1 | 2 | 3 | 4 | 5:
        kind = "weekday"
    case 6 | 7:
        kind = "weekend"
    case _:  # _ means "anything else"
        kind = "not a day"
print(kind)
