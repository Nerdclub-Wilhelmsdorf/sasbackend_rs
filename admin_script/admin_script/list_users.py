
import json
from surrealdb import Surreal

from admin_script import main


async def list_users():
    async with Surreal("wss://banking.saswdorf.de:8000/rpc") as db:
        await db.signin({"user": main.DBUSER, "pass": main.DBPSSWD})
        await db.use("user", "user")
        ids = await db.query("SELECT id FROM user", {}) 
        print(ids[0]["result"])
