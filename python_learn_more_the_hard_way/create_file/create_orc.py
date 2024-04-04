import pyarrow as pa
import pyarrow.orc as orc
import sys

filename = sys.argv[1]

# Define the schema
# Define the schema
schema = pa.schema([
    ('campaign_id', pa.int64()),
    ('campaign_name', pa.string()),
    ('action_type', pa.string()),
    ('timestamps', pa.int64()),
    ('user_id', pa.string()),
    ('lat', pa.float64()),
    ('lon', pa.float64())
])

# data
data = {
    'campaign_id': [int(sys.argv[2])],
    'campaign_name': [sys.argv[3]],
    'action_type': [sys.argv[4]],
    'timestamps': [int(sys.argv[5])],
    'user_id': [sys.argv[6]],
    'lat': [float(sys.argv[7])],
    'lon': [float(sys.argv[8])]
}

table = pa.Table.from_pydict(data, schema=schema)

with pa.OSFile(f'tmp/{filename}.orc.snappy', 'wb') as file:
    orc.write_table(table, file, compression='snappy')
