from surrealdb import Surreal
from admin_script import main


async def verify_user():
    id = input("Enter Account ID: ")
    name = input("Enter Student Number: ")
    async with Surreal("ws://localhost:8000/rpc") as db:
        await db.signin({"user": main.DBUSER, "pass": main.DBPSSWD})
        await db.use("user", "user")
        user = await db.select("user:" + id)
    if main.matchpswd(name, user["name"]):
        green = '\033[92m'
        end = '\033[0m'
        print(f"\n{green}User verified successfully\n{end}")
    else:
        red = '\033[91m'
        end = '\033[0m'
        print(f"\n{red}User verification failed\n{end}")