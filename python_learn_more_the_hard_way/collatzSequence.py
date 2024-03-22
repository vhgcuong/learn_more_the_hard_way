import time

try:
    number = int(input(f'Nhập số nguyên: '))
except ValueError:
    print(f'Vui long nhap so nguyen')

def collatz(number):
    print(f'{number}')
    time.sleep(0.1)
    if number == 1:
        return number
    if number % 2 == 0:
        collatz(number // 2)
    else:
        collatz(3 * number + 1)

collatz(number)
