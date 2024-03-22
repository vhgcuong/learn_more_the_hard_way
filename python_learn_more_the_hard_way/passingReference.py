def eggs(someParameter):
    someParameter.append('Hello')

spam = [1, 2, 3]
eggs(spam)
print(spam)

import copy
spam = ['A', 'B', 'C', 'D']
print(id(spam))

cheese = copy.copy(spam)
print(id(cheese))

