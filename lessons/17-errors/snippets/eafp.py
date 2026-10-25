class ConfigError(Exception):
    pass


def threads(data):
    try:
        return int(data["threads"])
    except KeyError as e:
        raise ConfigError("the threads setting is missing") from e
    except ValueError as e:
        raise ConfigError("the threads setting is not a number") from e


for data in [{"threads": "8"}, {}, {"threads": "many"}]:
    try:
        print("threads:", threads(data))
    except ConfigError as e:
        print("error:", e, "| cause:", repr(e.__cause__))
