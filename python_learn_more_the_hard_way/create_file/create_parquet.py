import pyarrow as pa
import pyarrow.parquet as pq
import pandas as pd

# Dữ liệu mẫu
data = {
    "name": ["Alice", "Bob", "Charlie"],
    "age": [30, 35, 40],
    "city": ["New York", "Los Angeles", "Chicago"]
}

# Tạo DataFrame từ dữ liệu mẫu
df = pd.DataFrame(data)

# Chuyển đổi DataFrame thành bảng Arrow
table = pa.Table.from_pandas(df)

# Ghi dữ liệu vào tệp Parquet
pq.write_table(table, 'tmp/data.parquet')


