import unittest


def to_fahrenheit(celsius):
    return celsius * 9 / 5 + 32


class TemperatureTests(unittest.TestCase):
    def test_boiling_point(self):
        self.assertEqual(to_fahrenheit(100), 212)

    def test_freezing_point(self):
        self.assertEqual(to_fahrenheit(0), 32)


if __name__ == "__main__":
    unittest.main()
