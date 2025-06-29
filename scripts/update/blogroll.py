# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "feedparser",
#     "opml",
#     "python-dateutil",
# ]
# ///

import argparse
import logging
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path

import feedparser
import opml
from dateutil import parser as date_parser

# Minimum date for sorting (UTC-aware)
MIN_DATE = datetime(1, 1, 1, tzinfo=timezone.utc)

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S",
)


@dataclass
class Feed:
    title: str
    xml_url: str


@dataclass
class Entry:
    title: str
    link: str
    date: datetime | None


def parse_opml(file_path: Path) -> list[Feed]:
    """Parse an OPML file and return a list of Feed objects, flattening nested outlines."""
    outlines = opml.parse(file_path)
    feeds: list[Feed] = []

    def walk(items):
        for item in items:
            xml_url = getattr(item, "xmlUrl", None)
            title = getattr(item, "title", None) or getattr(item, "text", None)
            if xml_url:
                feeds.append(Feed(title=title or xml_url, xml_url=xml_url))
            if hasattr(item, "outline"):
                walk(item.outline)

    walk(outlines)
    logging.info(f"Found {len(feeds)} feeds in '{file_path}'")
    return feeds


def get_entry_datetime(entry) -> datetime | None:
    """Extract or parse a datetime from a feed entry, normalized to UTC-aware."""
    dt = None
    # Try structured dates first
    if getattr(entry, "published_parsed", None):
        dt = datetime(*entry.published_parsed[:6], tzinfo=timezone.utc)
    else:
        date_str = entry.get("date")
        if date_str:
            try:
                dt = date_parser.parse(date_str)
            except (ValueError, TypeError) as e:
                logging.debug(f"Could not parse date string '{date_str}': {e}")
    if dt:
        # Ensure UTC awareness
        if dt.tzinfo:
            return dt.astimezone(timezone.utc)
        return dt.replace(tzinfo=timezone.utc)
    return None


def parse_feed(feed: Feed, max_entries: int = 3) -> list[Entry]:
    """Fetch and parse feed entries, filter invalid ones, and return top N by date."""
    logging.info(f"Parsing feed: {feed.title} ({feed.xml_url})")
    try:
        parsed = feedparser.parse(feed.xml_url)
        if parsed.bozo:
            logging.warning(f"Error parsing '{feed.xml_url}': {parsed.bozo_exception}")

        entries: list[Entry] = []
        for raw in parsed.entries:
            link = raw.get("link")
            if not link:
                continue
            dt = get_entry_datetime(raw)
            title = raw.get("title") or raw.get("author") or feed.title
            entries.append(Entry(title=title, link=link, date=dt))

        # Sort entries: newest first, None dates at the end
        entries.sort(key=lambda e: e.date if e.date else MIN_DATE, reverse=True)
        return entries[:max_entries]
    except Exception as e:
        logging.error(f"Failed to fetch or parse '{feed.xml_url}': {e}")
        return []


def write_markdown(entries: list[Entry], output_path: Path, frontmatter_title: str):
    """Write entries to a markdown file with YAML frontmatter."""
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with output_path.open("w", encoding="utf-8") as f:
        f.write(f'---\ntitle: "{frontmatter_title}"\n---\n\n')
        for entry in entries:
            f.write(f"- [{entry.title}]({entry.link})\n")


def main():
    parser = argparse.ArgumentParser(description="Update blogroll from an OPML file")
    parser.add_argument(
        "-i",
        "--input",
        type=Path,
        default=Path("assets/blogroll/index.opml"),
        help="Path to the OPML input file",
    )
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        default=Path("assets/blogroll/index.md"),
        help="Path to the Markdown output file",
    )
    parser.add_argument("-n", "--num", type=int, default=3, help="Max entries per feed")
    parser.add_argument(
        "-t", "--title", type=str, default="/blogroll/", help="YAML frontmatter title"
    )
    parser.add_argument(
        "-v", "--verbose", action="store_true", help="Enable debug logging"
    )
    args = parser.parse_args()

    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)

    feeds = parse_opml(args.input)
    all_entries: list[Entry] = []

    # Fetch feeds concurrently
    with ThreadPoolExecutor() as executor:
        futures = {executor.submit(parse_feed, feed, args.num): feed for feed in feeds}
        for future in as_completed(futures):
            all_entries.extend(future.result())

    # Final sorting across all feeds
    all_entries.sort(key=lambda e: e.date if e.date else MIN_DATE, reverse=True)
    write_markdown(all_entries, args.output, args.title)
    logging.info(f"Wrote {len(all_entries)} entries to '{args.output}'")


if __name__ == "__main__":
    main()
