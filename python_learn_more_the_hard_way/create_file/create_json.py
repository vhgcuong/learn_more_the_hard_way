import json
import gzip

# Dữ liệu mẫu
data = {
    "name": "John",
    "age": 30,
    "city": "New York"
}

# Tạo và ghi dữ liệu vào tệp JSON
with open('tmp/data.json', 'w') as json_file:
    json.dump(data, json_file)

# Chuyển đổi dữ liệu thành chuỗi JSON
json_data = json.dumps(data)

# Nén chuỗi JSON bằng gzip
with gzip.open('tmp/data.json.gz', 'wb') as json_gzip_file:
    json_gzip_file.write(json_data.encode('utf-8'))


