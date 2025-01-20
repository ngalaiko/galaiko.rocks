# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "python-slugify",
# ]
# ///
import os
import time
import argparse
from urllib.parse import urljoin
import json
from slugify import slugify
import urllib.request

BASE_URL = "https://api.discogs.com"
USER_AGENT = "nikita.galaiko.rocks carwler"


def get_page(token, page_url):
    request = urllib.request.Request(
        method="GET",
        url=page_url,
        headers={
            "User-Agent": USER_AGENT,
            "Authorization": f"Discogs token={token}",
            "Accept": "application/json",
        },
    )
    response = urllib.request.urlopen(request).read()
    return json.loads(response)


def get_image(url, token):
    while True:
        request = urllib.request.Request(
            method="GET",
            url=url,
            headers={
                "User-Agent": USER_AGENT,
                "Authorization": f"Discogs token={token}",
            },
        )
        response = urllib.request.urlopen(request)
        if response.status == 429:  # Too Many Requests
            retry_after = int(response.headers.get("Retry-After", 1))
            time.sleep(retry_after)
            continue
        return response.read()


def main(username, token, output):
    if not os.path.exists(output):
        os.makedirs(output)

    records = []
    page_url = urljoin(
        BASE_URL, f"/users/{username}/collection/folders/0/releases?sort=artist"
    )

    while page_url:
        page = get_page(token, page_url)
        records.extend(page["releases"])
        page_url = page["pagination"]["urls"].get("next")

    all_files = []

    for record in records:
        title_slug = slugify(record["basic_information"]["title"])

        cover_image = record["basic_information"]["cover_image"]
        filename, ext = os.path.splitext(os.path.basename(cover_image))

        if filename != "spacer":
            image_out = os.path.join(output, f"{title_slug}{ext}")

            if not os.path.exists(image_out):
                response = get_image(cover_image, token)
                with open(image_out, "wb") as f:
                    f.write(response)
            all_files.append(image_out)

        output_json = os.path.join(output, f"{title_slug}.json")

        if not os.path.exists(output_json):
            with open(output_json, "w") as f:
                json.dump(record, f, indent=4)
        all_files.append(output_json)

    for root, _, files in os.walk(output):
        for file in files:
            file_path = os.path.join(root, file)
            if file_path not in all_files:
                os.remove(file_path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Scrape Letterboxd diary entries.")
    parser.add_argument(
        "-o", "--output", help="Output directory", default="./assets/records/"
    )
    parser.add_argument("-t", "--token", help="Discogs token")
    parser.add_argument("-u", "--username",
                        help="Discogs username", default="ngalaiko")
    args = parser.parse_args()

    main(args.username, args.token, args.output)
