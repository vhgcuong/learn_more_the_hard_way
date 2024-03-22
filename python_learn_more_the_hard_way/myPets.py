myPets = ['Zophie', 'Pooka', 'Fat-tail']
print(f'Enter a pet name:')
name = input()
if name not in myPets:
    print(f'I do not have a pet named {name}')
else:
    print(f'{name} is my pet.')

import random

pets = ['Dog', 'Cat', 'Moose']
print(f'{random.choice(pets)}')

people = ['Alice', 'Bob', 'Carol', 'David']
print(f'{random.choice(people)}')

bacon = ['Zophie']
bacon *= 3
print(bacon)


spam = ['ants', 'cats', 'dogs', 'badgers', 'elephants']
print(spam)
spam.sort()
print(f'Sort: {spam}')

spam = ['apples',
    'oranges',
                    'bananas',
'cats']
print(spam)

print(id(spam))
