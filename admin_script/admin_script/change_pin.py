import json
from surrealdb import Surreal
from admin_script import main


async def change_pin():
    id = input("Enter Account ID: ")
    name = input("Enter Student Number: ")
    pin = input("Enter New Pin: ")
    async with Surreal("wss://banking.saswdorf.de:8000/rpc") as db:
        await db.signin({"user": main.DBUSER, "pass": main.DBPSSWD})
        await db.use("user", "user")
        user = await db.select(f'user:{id}')
        user = user["pin"]
        if main.matchpswd(name, user) == False:
            print("Invalid Student Number")
            return
        print(user)
        res = await db.query(f'UPDATE {"user:" + id} SET pin = "{main.hashb(pin)}"',{
        })
        green = '\033[92m'
        red = '\033[91m'
        if res[0]["status"] == "OK":
            print(f"\n{green}Pin changed successfully\n{red}")
            return
        print(f"\n{red}{res[0]["result"]}\n{red}")

