from repro_bug import empty_df
import time

start = time.perf_counter()
df = empty_df()
end = time.perf_counter()
print(df)
print(f"time taken for first: {1000 * (end - start):.2f} ms")

start = time.perf_counter()
df = empty_df()
end = time.perf_counter()
print(df)
print(f"time taken for second: {1000 * (end - start):.2f} ms")
