#!/usr/bin/env python3

import sys
import os
import json
from urllib import request, parse

def fetch_releases(token, page = 1):
    discogs_base_url = "https://api.discogs.com"

    url = discogs_base_url + "/users/ngalaiko/collection/folders/0/releases?sort=artist&page={}".format(page)
    req = request.Request(url, headers={
        "Content-Type": "application/json",
        "Authorization": "Discogs token={}".format(token),
    })
    resp = request.urlopen(req).read()
    releases_resp = json.loads(resp)
    releases = releases_resp['releases']

    pagination = releases_resp['pagination']
    if not 'next' in pagination['urls']:
        return releases

    return releases + fetch_releases(token, page=page+1)

def save_releases(releases):
    script_location = os.path.realpath(__file__)
    script_dir = os.path.dirname(script_location)
    dst = os.path.join(script_dir, "../../data/records.json")
    dst = os.path.normpath(dst)

    content = json.dumps({'releases': releases}, sort_keys=True, indent=2)
    with open(dst, 'w') as f:
        f.write(content)

if __name__ == "__main__":
    if len(sys.argv) != 2: 
        print("usage: {} <discogs_token>".format(sys.argv[0]))
        sys.exit(1)

    token = sys.argv[1]
    releases = fetch_releases(token)
    save_releases(releases)
