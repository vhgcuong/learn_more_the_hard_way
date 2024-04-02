import json
import gzip

# Dữ liệu mẫu
data = [
    {
        "camp_id": "1",
        "camp_name": "camp1",
        "type": "a",
        "sec": "1",
        "user_id": "user1",
        "client_id": "client1",
        "lat": 35.900242850000000,
        "lon": 139.897993400000020,
    },
    {
        "camp_id": "2",
        "camp_name": "camp2",
        "type": "b",
        "sec": "2",
        "user_id": "user2",
        "client_id": "client2",
        "lat": 35.900242850000000,
        "lon": 139.897993400000020,
    },
    {
        "camp_id": "3",
        "camp_name": "camp3",
        "type": "c",
        "sec": "3",
        "user_id": "user3",
        "client_id": "client3",
        "lat": 35.900242850000000,
        "lon": 139.897993400000020,
    },
    {
        "camp_id": "4",
        "camp_name": "camp4",
        "type": "d",
        "sec": "4",
        "user_id": "user4",
        "client_id": "client4",
        "lat": 35.900242850000000,
        "lon": 139.897993400000020,
    },
    {
        "camp_id": "5",
        "camp_name": "camp5",
        "type": "e",
        "sec": "5",
        "user_id": "user5",
        "client_id": "client5",
        "lat": 35.900242850000000,
        "lon": 139.897993400000020,
    },
]

# Tạo và ghi dữ liệu vào tệp JSON
with open('tmp/data.json', 'w') as json_file:
    json.dump(data, json_file)

# Chuyển đổi dữ liệu thành chuỗi JSON
json_data = json.dumps(data)

# Nén chuỗi JSON bằng gzip
with gzip.open('tmp/data.json.gz', 'wb') as json_gzip_file:
    json_gzip_file.write(json_data.encode('utf-8'))


