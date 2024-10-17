from surrealdb import Surreal
from admin_script import main
async def create_user():
    id = input("Enter Account ID (Random): ")
    if id == "":
        id = random_string(12)
    name = input("Enter Student Number: ")
    balance = input("Enter Balance (0): ")
    if balance == "":
        balance = 0
    pin = input("Enter Pin (Random): ")
    if pin == "":
        pin = random4int()
    guest = input("Is this a guest account? (y/n): ")
    is_guest = False
    if guest == "y":
        is_guest = True
    else:
        is_guest = False
    await new_user(id, name, balance, pin, is_guest)

async def new_user(id, name, balance, pin, is_guest):
    print("Creating user...")
    print("ID: " + id)
    print("Name: " + name)
    print("Balance: " + str(balance))
    print("Pin: " + str(pin))
    print("Guest: " + str(is_guest))
    async with Surreal("wss://banking.saswdorf.de:8000/rpc") as db:
        await db.signin({"user": main.DBUSER, "pass": main.DBPSSWD})
        await db.use("user", "user") 
        if id == "":
           created_record = await db.create(
                "user",
                {
                    "balance": balance,
                    "pin": main.hashb(pin),
                    "name": main.hashb(name),
                    "transactions": "",
                    "guest" : is_guest
                },
            )
           id = created_record[0]["id"]
        else:
            await db.create(
                "user",
                {
                    "id": id,
                    "balance": balance,
                    "pin": main.hashb(pin),
                    "name": main.hashb(name),
                    "transactions": "",
                    "guest" : is_guest
                },
            )
            id = "user:" + id
    green = '\033[92m'
    end = '\033[0m'
    print(f"\n{green}User created successfully\n{end}")
    print(f"ID: {id}")
    print(f"PIN: {pin}")
def random4int():
    n = 4
    import secrets
    range_start = 10**(n-1)
    range_end = (10**n) - 1
    return secrets.randbelow(range_end - range_start + 1) + range_start


def random_string(length):
    import random
    import string
    return ''.join(random.choices(string.ascii_uppercase + string.digits + string.ascii_lowercase, k=length))