# jshape for Python

`jshape` is the Python package for the Rust-powered JSON shape analyzer in this repository.

## Install

```bash
pip install jshape
```

## Usage

```python
import jshape

outline = jshape.analyze_json(
    '{"user":{"name":"Ada"},"events":[{"id":1},{"id":2,"amount":19.9}]}',
    True,
)

print(outline)
```
