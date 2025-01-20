# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "beautifulsoup4",
# ]
# ///
from bs4 import BeautifulSoup
import argparse
import json
import urllib.request
import os
from datetime import datetime


def main(username, output):
    n = 1
    entries = []

    while True:
        page_entries, has_next = fetch_page(username, n)
        entries.extend(page_entries)
        if has_next:
            n += 1
        else:
            break

    all_files = []
    for entry in entries:

        def parse_watch_number(href):
            try:
                return int(href.split("/")[-2])
            except Exception:
                return 0

        watch_number = parse_watch_number(entry["href"])
        watch_number = (
            watch_number if watch_number < 10 else 0
        )  # because movie 1917 exists
        data_output = os.path.join(output, entry["title_slug"], f"{watch_number}.json")

        os.makedirs(os.path.dirname(data_output), exist_ok=True)

        if not os.path.exists(data_output):
            with open(data_output, "w") as f:
                json.dump(entry, f, indent=4, default=str)

        poster_output = os.path.join(output, f"{entry['title_slug']}.jpg")

        if not os.path.exists(poster_output):
            image = get_poster(entry)
            with open(poster_output, "wb") as f:
                f.write(image)

        all_files.append(data_output)
        all_files.append(poster_output)

    for root, dirs, files in os.walk(output):
        for file in files:
            file_path = os.path.join(root, file)
            if file_path not in all_files:
                os.remove(file_path)


def fetch_page(username, n):
    request = urllib.request.Request(
        method="GET",
        url=f"https://letterboxd.com/{username}/films/diary/page/{n}/",
        headers={"User-Agent": "nikita.galaiko.rocks robot"},
    )
    response = urllib.request.urlopen(request).read()
    return parse_page(response)


def parse_page(body):
    soup = BeautifulSoup(body, "html.parser")
    entries = [
        parse_entry(entry)
        for entry in soup.find_all(attrs={"data-object-name": ["entry", "review"]})
    ]
    has_next = bool(soup.find("a", class_="next"))
    return entries, has_next


def parse_entry(entry):
    date_node = entry.find("td", class_="td-day").find("a")
    date_str = date_node["href"].split("/")[-4:-1]
    date = datetime.strptime("-".join(date_str), "%Y-%m-%d").date()

    title = entry.find("td", class_="td-film-details").find("h3").find("a").text

    is_liked = bool(entry.find("td", class_="td-like").find(class_="icon-liked"))
    is_rewatch = bool(
        entry.find("td", class_="td-rewatch").find(class_="icon-status-off") is None
    )

    details = entry.find("td", class_="td-actions")
    href = (
        "https://letterboxd.com"
        + entry.find("td", class_="td-film-details").find("h3").find("a")["href"]
    )
    title_slug = details["data-film-slug"]

    return {
        "title": title,
        "title_slug": title_slug,
        "date": date,
        "is_rewatch": is_rewatch,
        "is_liked": is_liked,
        "href": href,
    }


def get_poster(entry):
    request = urllib.request.Request(
        method="GET",
        url=f"https://letterboxd.com/film/{entry['title_slug']}/",
        headers={"User-Agent": "nikita.galaiko.rocks robot"},
    )
    response = urllib.request.urlopen(request).read()
    soup = BeautifulSoup(response, "html.parser")

    ld_data = soup.find("script", type="application/ld+json").string
    ld_data = ld_data.split("\n")[2]
    ld_data = json.loads(ld_data)
    image_request = urllib.request.Request(
        method="GET",
        url=ld_data["image"].replace("0-230-0-345", "0-600-0-900"),
        headers={"User-Agent": "nikita.galaiko.rocks robot"},
    )
    return urllib.request.urlopen(image_request).read()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Scrape Letterboxd diary entries.")
    parser.add_argument(
        "-o", "--output", help="Output directory", default="./assets/movies"
    )
    parser.add_argument(
        "-u", "--username", help="Letterboxd username", default="ngalaiko"
    )
    args = parser.parse_args()

    main(args.username, args.output)
