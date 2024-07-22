# pyo3-polars bug

## Steps to reproduce

1. `maturin develop -r`
2. `pip install polars`
3. `python main.py`

## Output:

```bash
❯ python main.py
shape: (0, 0)
┌┐
╞╡
└┘
time taken for first: 42.98 ms
shape: (0, 0)
┌┐
╞╡
└┘
time taken for second: 0.02 ms
```
