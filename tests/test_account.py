import piledger

x = piledger.Account()
print(f"{x} of type {type(x)}")

x.number = 1002
print(f"{x} of type {type(x)}")

y = piledger.Account()

print(y.acctname)
y.acctname="PlaceHolder2"
print(y.acctname)

try:
    x.number = "invalid"
except Exception as e:
    print(f"Caught expected exception: {e}")