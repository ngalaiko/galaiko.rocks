# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "feedparser",
#     "opml",
# ]
# ///

import argparse
import feedparser
import opml
import time


def parse_opml(file_path):
    """Parse an OPML file and return a list of feeds."""
    with open(file_path, "r") as file:
        return opml.parse(file)


def truncate_date(date: time.struct_time) -> time.struct_time:
    if not date:
        return None
    """Truncate a date to the nearest day."""
    return time.struct_time(
        (
            date.tm_year,
            date.tm_mon,
            date.tm_mday,
            0,  # hour
            0,  # minute
            0,  # second
            date.tm_wday,  # weekday
            date.tm_yday,  # yearday
            -1,  # isdst
        )
    )


def get_entry_date(entry):
    """Get the date of an entry, preferring 'updated_parsed' over 'published_parsed'."""
    if "updated_parsed" in entry:
        return truncate_date(entry["updated_parsed"])
    elif "published_parsed" in entry:
        return truncate_date(entry["published_parsed"])
    else:
        return None


def sort_entries(entries):
    """Sort entries by date, with None dates at the end."""
    return sorted(
        entries,
        key=lambda x: get_entry_date(x) if get_entry_date(x) else "",
        reverse=True,
    )


def parse_feed(feed, max_entries=3):
    """Parse a feed and return a list of entries."""
    print(f"Parsing {feed.title} ({feed.xmlUrl})")
    feed = feedparser.parse(feed.xmlUrl)
    entries = feed.entries[:max_entries]

    def filter_entry(entry):
        if "link" not in entry:
            return False
        if "title" not in entry and "author" not in entry:
            return False
        if not get_entry_date(entry):
            return False
        return True

    def map_entry(entry):
        return {
            "title": entry.get("title", entry.get("author", ""), feed.title),
            "link": entry["link"],
            "date": get_entry_date(entry),
        }

    entries = filter(filter_entry, entries)
    entries = map(map_entry, entries)

    return sort_entries(entries)[:max_entries]


def main(input_file, output_file):
    feeds = parse_opml(input_file)
    entries = [entry for feed in feeds for entry in parse_feed(feed)]
    entries = sort_entries(entries)

    with open(output_file, "w") as file:
        file.write(
            """---
title: /blogroll/
---
"""
        )
        date = None
        for entry in entries:
            if entry["date"] != date:
                date = entry["date"]
                file.write(
                    f"\n## {date.tm_year}-{date.tm_mon:02d}-{date.tm_mday:02d}\n\n"
                )
            title = entry["title"]
            link = entry["link"]
            file.write(f"- [{title}]({link})\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Update blogroll")
    parser.add_argument(
        "-o",
        "--output",
        help="Output file",
        default="./assets/blogroll/index.md",
    )
    parser.add_argument(
        "-i",
        "--input",
        help="Input file",
        default="./assets/blogroll/index.opml",
    )
    args = parser.parse_args()

    main(args.input, args.output)
