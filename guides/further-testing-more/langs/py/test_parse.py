import unittest

from parse import seconds


class SecondsTable(unittest.TestCase):
    def test_valid(self):
        cases = [("90s", 90), ("2m", 120), ("1h30m", 5400), ("1h1m1s", 3661)]
        for text, want in cases:
            with self.subTest(text=text):
                self.assertEqual(seconds(text), want)

    def test_invalid(self):
        for text in ["", "15", "m", "5x"]:
            with self.subTest(text=text):
                with self.assertRaises(ValueError):
                    seconds(text)


if __name__ == "__main__":
    unittest.main()
