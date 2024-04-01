import pyarrow as pa
import pyarrow.orc as orc
import gzip
import io

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
    'camp_id': ["1", "2", "3", "4", "5"],
    'camp_name': ["camp1", "camp2", "camp3", "camp4", "camp5"],
    'type': ["a", "b", "c", "d", "e"],
    'sec': ["1", "2", "3", "4", "5"],
    'user_id': ["user1", "user2", "user3", "user4", "user5"],
    'client_id': ["client1", "client2", "client3", "client4", "client5"],
    'lat': [35.900242850000000, 35.900242850000000, 35.900242850000000, 35.900242850000000, 35.900242850000000],
    'lon': [139.897993400000020, 139.897993400000020, 139.897993400000020, 139.897993400000020, 139.897993400000020]
}
table = pa.Table.from_pydict(data, schema=schema)

buffer = io.BytesIO()
with gzip.GzipFile(fileobj=buffer, mode='wb') as gzip_file:
    orc.write_table(table, gzip_file)

# Lưu nội dung từ bộ đệm vào tệp tin
with open('tmp/data.orc.gz', 'wb') as file:
    file.write(buffer.getvalue())
