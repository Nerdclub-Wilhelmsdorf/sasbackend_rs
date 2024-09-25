from surrealdb import Surreal
from admin_script import main

async def create_user():
    id = input("Enter Account ID (Random): ")
    name = input("Enter Student Number: ")
    balance = input("Enter Balance (0): ")
    if balance == "":
        balance = 0
    pin = input("Enter Pin (Random): ")
    if pin == "":
        pin = random4int()
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




