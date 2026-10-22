# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "python-slugify",
# ]
# ///
import csv
from slugify import slugify
import subprocess
import os
import argparse
import json
import datetime
import io
from collections import defaultdict


def main(file, output):
    output_dir = os.path.dirname(output)
    os.makedirs(output_dir, exist_ok=True)

    today = datetime.date.today()
    year_ago = today - datetime.timedelta(days=365)
    year_ago_str = year_ago.strftime("%Y-%m-%d")

    command = [
        "ledger",
        "csv",
        "--exchange=SEK",
        "expenses:Food:Lunch$",
        "expenses:Food:Eating Out$",
        f"--begin={year_ago_str}",
        f"--end={today + datetime.timedelta(days=1)}",
        "--format",
        '%(quoted(date)),%(quoted(payee)),%(quoted(commodity(scrub(display_amount)))),'
        '%(quoted(quantity(scrub(display_amount)))),%(quoted(tag("Latitude"))),'
        '%(quoted(tag("Longitude")))\\n',
    ]

    if file:
        command.append(f"--file={file}")

    try:
        ledger_output = subprocess.check_output(command, stderr=subprocess.STDOUT)
    except subprocess.CalledProcessError as e:
        raise Exception(e.output.decode())

    rows = list(csv.reader(io.StringIO(ledger_output.decode())))

    entries = []
    for date, raw_payee, commodity, amount, latitude, longitude in rows:
        payee = raw_payee.split("|")[0].strip()
        if payee == "Commodities revalued":
            continue
        entry = {
            "date": date, "amount": float(amount), "payee": payee,
            "commodity": commodity, "location": (latitude, longitude),
        }
        entries.append(entry)

    entries_by_place = defaultdict(list)
    for entry in entries:
        entries_by_place[entry["payee"]].append(entry)

    places = []
    for payee, entries in entries_by_place.items():
        entries_by_date = defaultdict(list)
        for entry in entries:
            entries_by_date[entry["date"]].append(entry)
        times = len(entries_by_date)
        if times < 2:
            continue
        # Ledger's currency-rounding adjustments have no tags of their own.
        coordinates = {entry["location"] for entry in entries if any(entry["location"])}
        if not coordinates:
            raise ValueError(f"Location for '{payee}' is missing or invalid")
        locations = set()
        for coordinate in coordinates:
            try:
                location = tuple(float(value) for value in coordinate)
            except ValueError:
                raise ValueError(f"Location for '{payee}' is missing or invalid") from None
            if not (-90 <= location[0] <= 90 and -180 <= location[1] <= 180):
                raise ValueError(f"Location for '{payee}' is out of range")
            locations.add(location)
        if len(locations) != 1:
            raise ValueError(f"Conflicting locations for '{payee}'")
        location = locations.pop()
        place = {
            "location": location,
            "name": payee,
            "times": len(entries_by_date),
            "spent": sum(entry["amount"] for entry in entries),
        }
        places.append((payee, place))

    places.sort(key=lambda x: x[1]["times"], reverse=True)

    all_files = []
    for _, place in places:
        output_file = os.path.join(output_dir, f"{slugify(place['name'])}.json")
        all_files.append(output_file)
        with open(output_file, "w") as f:
            json.dump(place, f, indent=4)

    for root, dirs, files in os.walk(output):
        for file in files:
            file_path = os.path.join(root, file)
            if file_path not in all_files:
                os.remove(file_path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Export restaurant and cafe visits from Ledger.")
    parser.add_argument("-f", "--file", help="Ledger journal")
    parser.add_argument(
        "-o", "--output", help="Output directory", default="./assets/places/"
    )
    args = parser.parse_args()

    main(args.file, args.output)
