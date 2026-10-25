import duckdb
import datafusion
import pandas as pd
import polars as pl
import pyarrow as pa
import pyarrow.parquet as pq

# The same five rows as the Rust program, written to a Parquet file by PyArrow.
pq.write_table(
    pa.table({
        "city": ["Pune", "Pune", "Delhi", "Delhi", "Chennai"],
        "month": [1, 2, 1, 2, 1],
        "sales": [120.0, 135.5, 210.0, 190.25, 99.0],
    }),
    "sales.parquet",
)

# Total sales per city, biggest first, with each library.
results = {}

pdf = pd.read_parquet("sales.parquet").groupby("city")["sales"].sum().sort_values(ascending=False)
results["pandas"] = list(pdf.items())

plf = pl.read_parquet("sales.parquet").group_by("city").agg(pl.col("sales").sum().alias("total")).sort("total", descending=True)
results["polars"] = plf.rows()

results["duckdb"] = duckdb.sql(
    "SELECT city, SUM(sales) AS total FROM 'sales.parquet' GROUP BY city ORDER BY total DESC"
).fetchall()

ctx = datafusion.SessionContext()
ctx.register_parquet("sales", "sales.parquet")
batches = ctx.sql("SELECT city, SUM(sales) AS total FROM sales GROUP BY city ORDER BY total DESC").collect()
results["datafusion"] = [row for b in batches for row in zip(b.column("city").to_pylist(), b.column("total").to_pylist())]

for name, rows in results.items():
    print(f"{name:<10}", [(c, float(t)) for c, t in rows])
print("all agree:", len({str([(c, float(t)) for c, t in r]) for r in results.values()}) == 1)
print("versions:", ", ".join(f"{m.__name__} {m.__version__}" for m in (pd, pl, duckdb, pa, datafusion)))
