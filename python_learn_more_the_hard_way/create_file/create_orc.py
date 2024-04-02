import pandas as pd
import pyarrow as pa
import pyarrow.orc as orc
import gzip

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

# Create DataFrame
df = pd.DataFrame(data)

# Convert DataFrame to Arrow table
table = pa.Table.from_pandas(df, schema=schema)

# Write data to Orc file
with pa.OSFile('tmp/data.orc', 'wb') as file:
    orc.write_table(table, file)


#### gzip
#################################################################################
# Read the Orc file
with open('tmp/data.orc', 'rb') as orc_file:
    orc_data = orc_file.read()

# Compress the Orc data using gzip
compressed_data = gzip.compress(orc_data)

# Write the compressed data to a gzip file
with gzip.open('tmp/data.orc.gz', 'wb') as gzip_file:
    gzip_file.write(compressed_data)

