import piledger

x = piledger.Account("Compte courant", "Actif", 1001)
print(f"{x} of type {type(x)}")

x.number = 1002
print(f"{x} of type {type(x)}")

try:
    x.number = "invalid"
except Exception as e:
    print(f"Caught expected exception: {e}")