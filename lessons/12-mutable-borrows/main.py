def add_bonus(scores: list[int], bonus: int) -> None:
    for i in range(len(scores)):
        scores[i] += bonus  # lists are passed by reference, so this changes the caller's list
    scores.append(0)


scores = [10, 20]
add_bonus(scores, 5)
print(scores)
