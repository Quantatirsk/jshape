#!/usr/bin/env python3

import json
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: prepare_node_pkg.py <package.json>")

    package_json = Path(sys.argv[1])
    data = json.loads(package_json.read_text())

    existing_keywords = data.get("keywords", [])
    merged_keywords = []
    for keyword in existing_keywords + ["json", "schema", "shape", "repair", "wasm"]:
        if keyword not in merged_keywords:
            merged_keywords.append(keyword)

    data["name"] = "@quanthub/jshape"
    data["description"] = "Repair malformed JSON and render a stable, human-readable structural outline."
    data["homepage"] = "https://github.com/Quantatirsk/jshape"
    data["license"] = "MIT"
    data["repository"] = {
        "type": "git",
        "url": "git+https://github.com/Quantatirsk/jshape.git",
        "directory": "bindings/node",
    }
    data["bugs"] = {"url": "https://github.com/Quantatirsk/jshape/issues"}
    data["keywords"] = merged_keywords
    data["publishConfig"] = {"access": "public"}
    data["sideEffects"] = False

    package_json.write_text(json.dumps(data, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
