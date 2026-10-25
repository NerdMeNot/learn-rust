# every object is already a shared reference
log = []
other = log

log.append("from log")
other.append("from other")

print("log:", ", ".join(log))
print("same list:", str(log is other).lower())
