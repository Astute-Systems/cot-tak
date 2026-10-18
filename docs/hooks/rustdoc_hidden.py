"""MkDocs hook: make Rust code blocks written for rustdoc render cleanly.

The guide's Rust snippets are compiled as doctests (see ``crates/cottak/src/guide_tests.rs``),
so they use rustdoc conventions that a web page should not show:

* lines starting with ``# `` (or a lone ``#``) are hidden setup code, removed here;
* ``rust,no_run`` and similar info strings are reduced to ``rust`` so the highlighter
  recognises them.

Only fenced blocks whose language is ``rust`` are touched. Attributes such as ``#[derive]``
and ``#![allow]`` are not hidden lines and are kept.
"""

import re

FENCE = re.compile(r"^(?P<indent>[ \t]*)(?P<ticks>`{3,})(?P<info>[^`\n]*)$")


def _is_hidden(line: str) -> bool:
    stripped = line.lstrip()
    return stripped == "#" or stripped.startswith("# ")


def on_page_markdown(markdown, **kwargs):
    out = []
    in_rust = False
    open_ticks = ""
    in_other = False
    for line in markdown.split("\n"):
        m = FENCE.match(line)
        if m and not in_rust and not in_other:
            info = m.group("info").strip()
            language = re.split(r"[,\s{]", info, maxsplit=1)[0] if info else ""
            if language == "rust":
                in_rust = True
                open_ticks = m.group("ticks")
                rest = info[len(language):]
                # Keep MkDocs attributes such as {title="x"}; drop rustdoc flags.
                attrs = rest[rest.index("{"):] if "{" in rest else ""
                out.append(f'{m.group("indent")}{open_ticks}rust{(" " + attrs) if attrs else ""}')
            else:
                in_other = True
                open_ticks = m.group("ticks")
                out.append(line)
            continue
        if m and (in_rust or in_other) and m.group("ticks") == open_ticks and not m.group("info").strip():
            in_rust = in_other = False
            out.append(line)
            continue
        if in_rust and _is_hidden(line):
            continue
        out.append(line)
    return "\n".join(out)
