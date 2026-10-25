day = 99
match day:
    case 1 | 2 | 3 | 4 | 5:
        kind = "weekday"
    case 6 | 7:
        kind = "weekend"
print(kind)
