import pandas as pd
import pyarrow as pa
import pyarrow.orc as orc

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

# Ghi dữ liệu vào tệp Orc
with pa.OSFile('tmp/data.orc', 'wb') as file:
    orc.write_table(table, file)
