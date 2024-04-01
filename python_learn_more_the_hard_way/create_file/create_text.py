import gzip

data = "John,30,New York"

# Mở hoặc tạo tệp văn bản mới với chế độ ghi ('w')
with open('tmp/data.txt', 'w') as file:
    # Ghi dữ liệu vào tệp
    file.write(data)

with gzip.open('tmp/data.txt.gz', 'wb') as file:
    file.write(data.encode('utf-8'))
