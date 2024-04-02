import gzip

data = [
    "1,camp1,a,1,user1,client1,35.900242850000000,139.897993400000020",
    "2,camp2,b,2,user2,client2,35.900242850000000,139.897993400000020",
    "3,camp3,c,3,user3,client3,35.900242850000000,139.897993400000020",
    "4,camp4,d,4,user4,client4,35.900242850000000,139.897993400000020",
    "5,camp5,e,5,user5,client5,35.900242850000000,139.897993400000020"
]

# Mở hoặc tạo tệp văn bản mới với chế độ ghi ('w')
# Write data to text file
with open('tmp/data.txt', 'w') as file:
    file.write('\n'.join(data))

# Compress text file using gzip
with gzip.open('tmp/data.txt.gz', 'wb') as file:
    file.write('\n'.join(data).encode('utf-8'))
