from piledger import AccountReader

print("AccountReader in for loop:")
for acc in AccountReader("tests/comptes.csv"):
    print(acc)

print("\nAccountReader with explicit iterator:")
my_iter = AccountReader("tests/comptes.csv").__iter__()

print(next(my_iter))
print(next(my_iter))
print(next(my_iter))
print(next(my_iter))