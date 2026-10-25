# A set of strings, like a Rust HashMap, has no insertion order.
# Python seeds its string hashes randomly for each process.
print({"one", "two", "three", "four", "five", "six"})
