# /// script
# requires-python = ">=3.13"
# dependencies = []
# ///
import xml.etree.ElementTree as ET
import argparse
import json
import re
import urllib.request
import os
from datetime import datetime


def main(username, output):
    rss = fetch_rss(username)
    entries = parse_rss(rss)

    for entry in entries:

        def parse_watch_number(href):
            try:
                return int(href.split("/")[-2])
            except Exception:
                return 0

        watch_number = parse_watch_number(entry["href"])
        title_slug = entry["title_slug"].replace(":", "_")
        watch_number = (
            watch_number if watch_number < 10 else 0
        )  # because movie 1917 exists
        data_output = os.path.join(output, title_slug, f"{watch_number}.json")

        os.makedirs(os.path.dirname(data_output), exist_ok=True)

        if not os.path.exists(data_output):
            with open(data_output, "w") as f:
                json.dump(entry, f, indent=4, default=str)

        poster_output = os.path.join(output, f"{title_slug}.jpg")

        if not os.path.exists(poster_output) and entry.get("poster_url"):
            image_request = urllib.request.Request(
                method="GET",
                url=entry["poster_url"],
                headers={"User-Agent": "nikita.galaiko.rocks robot"},
            )
            image = urllib.request.urlopen(image_request).read()
            with open(poster_output, "wb") as f:
                f.write(image)


def fetch_rss(username):
    request = urllib.request.Request(
        method="GET",
        url=f"https://letterboxd.com/{username}/rss/",
        headers={"User-Agent": "nikita.galaiko.rocks robot"},
    )
    return urllib.request.urlopen(request).read()


def parse_rss(body):
    root = ET.fromstring(body)
    items = root.findall(".//item")
    entries = []
    for item in items:
        entry = parse_item(item)
        if entry:
            entries.append(entry)
    return entries


def parse_item(item):
    ns = {"letterboxd": "https://letterboxd.com"}

    film_title_el = item.find("letterboxd:filmTitle", ns)
    if film_title_el is None:
        return None

    title = film_title_el.text
    link = item.findtext("link")

    watched_date_el = item.find("letterboxd:watchedDate", ns)
    date = (
        datetime.strptime(watched_date_el.text, "%Y-%m-%d").date()
        if watched_date_el is not None
        else None
    )

    is_rewatch = item.findtext("letterboxd:rewatch", default="No", namespaces=ns) == "Yes"
    is_liked = item.findtext("letterboxd:memberLike", default="No", namespaces=ns) == "Yes"

    # Extract title_slug from link: https://letterboxd.com/user/film/slug/...
    title_slug = None
    if link:
        match = re.search(r"/film/([^/]+)", link)
        if match:
            title_slug = match.group(1)

    if not title_slug:
        return None

    # Extract poster URL from description CDATA <img src="...">
    poster_url = None
    description = item.findtext("description") or ""
    img_match = re.search(r'<img\s+src="([^"]+)"', description)
    if img_match:
        poster_url = img_match.group(1).replace("0-230-0-345", "0-600-0-900")

    return {
        "title": title,
        "title_slug": title_slug,
        "date": date,
        "is_rewatch": is_rewatch,
        "is_liked": is_liked,
        "href": link,
        "poster_url": poster_url,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Fetch Letterboxd diary entries from RSS.")
    parser.add_argument(
        "-o", "--output", help="Output directory", default="./assets/movies"
    )
    parser.add_argument(
        "-u", "--username", help="Letterboxd username", default="ngalaiko"
    )
    args = parser.parse_args()

    main(args.username, args.output)
