import datetime
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


class LedgerExportTest(unittest.TestCase):
    def export(self, journal, directory):
        file = directory / "main.ledger"
        file.write_text(journal)
        output = directory / "places"
        output.mkdir(exist_ok=True)
        return subprocess.run(
            [sys.executable, str(Path(__file__).with_name("ledger.py")),
             "--file", str(file), "--output", str(output) + "/"],
            capture_output=True, text=True,
        )

    def test_coordinates_from_ledger_and_distinct_visit_dates(self):
        today = datetime.date.today()
        yesterday = today - datetime.timedelta(days=1)
        old = today - datetime.timedelta(days=366)
        tomorrow = today + datetime.timedelta(days=1)
        journal = f"""P 2000-01-01 EUR 10 SEK
payee Example Cafe
    alias ^Example Cafe($| *\\|)
= expr payee =~ /^Example Cafe$/
    ; Latitude: 57.1234
    ; Longitude: -11.5678
{yesterday} Example Cafe | lunch
    expenses:Food:Lunch  101.25 SEK
    assets:Cash
{yesterday} Example Cafe
    expenses:Food:Lunch  23.50 SEK
    assets:Cash
{today} Example Cafe
    expenses:Food:Eating Out  5 EUR
    assets:Cash
{today} Single Visit
    expenses:Food:Eating Out  70 SEK
    assets:Cash
{today} Single Visit
    expenses:Food:Lunch  10 SEK
    assets:Cash
{old} Old Visit
    expenses:Food:Lunch  20 SEK
    assets:Cash
{today} Old Visit
    expenses:Food:Lunch  20 SEK
    assets:Cash
{today} Future Visit
    expenses:Food:Lunch  20 SEK
    assets:Cash
{tomorrow} Future Visit
    expenses:Food:Lunch  20 SEK
    assets:Cash
{yesterday} Other Category
    expenses:Food:Restaurants & Cafes  20 SEK
    assets:Cash
{today} Other Category
    expenses:Food:Restaurants & Cafes  20 SEK
    assets:Cash
"""
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            result = self.export(journal, directory)
            self.assertEqual(result.returncode, 0, result.stderr)
            files = list((directory / "places").glob("*.json"))
            self.assertEqual(len(files), 1)
            self.assertEqual(json.loads(files[0].read_text()), {
                "name": "Example Cafe", "location": [57.1234, -11.5678],
                "times": 2, "spent": 174.75,
            })

    def test_invalid_qualifying_place_fails_without_changing_existing_places(self):
        today = datetime.date.today()
        cutoff = today - datetime.timedelta(days=365)
        for tags in ("", "; Latitude: 57", "; Latitude: north\n    ; Longitude: 12",
                     "; Latitude: 91\n    ; Longitude: 12",
                     "; Latitude: 57\n    ; Longitude: -181",
                     "; Latitude: nan\n    ; Longitude: 12"):
            with self.subTest(tags=tags), tempfile.TemporaryDirectory() as tmp:
                directory = Path(tmp)
                output = directory / "places"
                output.mkdir()
                existing = output / "existing.json"
                existing.write_text("unchanged")
                journal = f"""{cutoff} Missing Cafe
    {tags or '; No location recorded'}
    expenses:Food:Lunch  10 SEK
    assets:Cash
{today} Missing Cafe | dinner
    {tags or '; No location recorded'}
    expenses:Food:Eating Out  20 SEK
    assets:Cash
"""
                result = self.export(journal, directory)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("Location for 'Missing Cafe'", result.stderr)
                self.assertEqual(existing.read_text(), "unchanged")


if __name__ == "__main__":
    unittest.main()
