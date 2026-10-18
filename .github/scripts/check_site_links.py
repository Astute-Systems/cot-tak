#!/usr/bin/env python3
"""Fail if the assembled documentation site contains a broken relative link.

Usage: check_site_links.py <site-dir>

MkDocs validates links between guide pages, but the rustdoc output is merged in afterwards, so
links from the guide into ``api/`` (and rustdoc's own cross-links) are only checkable here.
Pages under ``api/`` are rustdoc's own output (including links it generates to other crates'
documentation) and are not checked; only links *into* ``api/`` from the guide are.
External links, ``mailto:``, ``data:`` URLs and bare fragments are ignored; fragments and query
strings are stripped before the file lookup.
"""

import pathlib
import sys
from html.parser import HTMLParser
from urllib.parse import unquote, urlsplit


class Links(HTMLParser):
    def __init__(self):
        super().__init__()
        self.links = []

    def handle_starttag(self, tag, attrs):
        for name, value in attrs:
            if name in ("href", "src") and value:
                self.links.append(value)


def main(root: pathlib.Path) -> int:
    broken = []
    checked = 0
    for page in root.rglob("*.html"):
        top = page.relative_to(root).parts[0]
        # 404.html uses site-absolute links, valid only under the deployed base path.
        if top == "api" or top == "404.html":
            continue
        parser = Links()
        parser.feed(page.read_text(encoding="utf-8", errors="replace"))
        for link in parser.links:
            parts = urlsplit(link)
            if parts.scheme or parts.netloc or not parts.path:
                continue  # external, mailto:, data:, //host, or a bare #fragment
            target = (page.parent / unquote(parts.path)).resolve()
            if target.is_dir():
                target = target / "index.html"
            checked += 1
            if not target.exists():
                broken.append(f"{page.relative_to(root)}: {link}")
    for line in sorted(set(broken)):
        print("BROKEN", line)
    print(f"checked {checked} relative links, {len(set(broken))} broken")
    return 1 if broken else 0


if __name__ == "__main__":
    sys.exit(main(pathlib.Path(sys.argv[1]).resolve()))
