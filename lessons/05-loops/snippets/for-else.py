def first_negative(xs):
    for x in xs:
        if x < 0:
            print("found a negative")
            break
    else:
        # runs only when the loop ended without a break
        print("none")


print("[3, -1, 4]:", end=" ")
first_negative([3, -1, 4])
print("[3, 1, 4]:", end=" ")
first_negative([3, 1, 4])
