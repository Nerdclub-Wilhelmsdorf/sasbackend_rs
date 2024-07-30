
from surrealdb import Surreal
from admin_script import main


async def delete_user():
    id = input("Enter Account ID: ")
    async with Surreal("ws://localhost:8000/rpc") as db:
        await db.signin({"user": main.DBUSER, "pass": main.DBPSSWD})
        await db.use("user", "user")
        await db.delete("user:" + id)
    green = '\033[92m'
    end = '\033[0m'
    print(f"\n{green}User deleted successfully\n{end}")