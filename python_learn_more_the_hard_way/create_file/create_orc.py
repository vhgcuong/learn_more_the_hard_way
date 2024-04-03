import pyarrow as pa
import pyarrow.orc as orc
import gzip
import io
import sys

filename = sys.argv[1]

# Define the schema
schema = pa.schema([
    ('camp_id', pa.string()),
    ('camp_name', pa.string()),
    ('type', pa.string()),
    ('sec', pa.string()),
    ('user_id', pa.string()),
    ('client_id', pa.string()),
    ('lat', pa.float64()),
    ('lon', pa.float64())
])

# Create a sample table
data = {
    'camp_id': [sys.argv[2]],
    'camp_name': [sys.argv[3]],
    'type': [sys.argv[4]],
    'sec': [sys.argv[5]],
    'user_id': [sys.argv[6]],
    'client_id': [sys.argv[7]],
    'lat': [float(sys.argv[8])],
    'lon': [float(sys.argv[9])]
}

table = pa.Table.from_pydict(data, schema=schema)

buffer = io.BytesIO()
with gzip.GzipFile(fileobj=buffer, mode='wb') as gzip_file:
    orc.write_table(table, gzip_file)

# Lưu nội dung từ bộ đệm vào tệp tin
with open(f'tmp/{filename}.orc.gz', 'wb') as file:
    file.write(buffer.getvalue())
