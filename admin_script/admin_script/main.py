#!/usr/bin/python3

from admin_script import change_pin, create_user, delete_user, get_logs, list_users, transfer, verify

DBPSSWD = "IE76qzUk0t78JGhTz"
DBUSER = "guffe"

async def main():
    print("Welcome to the admin console")
    print("the following commands are available:")
    print("[1] create - create a new account")
    print("[2] delete - delete an account")
    print("[3] list - list all accounts")
    print("[4] changepin - change the pin of an account")
    print("[5] verify - verify an account")
    print("[6] getlogs - get the logs of an account as CSV")
    print("[7] transaction - induce a transaction")
    print("[0] exit - exit the program")
    print("Please enter the number of the command you would like to run:")
    scanner = input()
    await matchInput(scanner)

async def matchInput(scanner):
    match scanner:
        case "1":
            await create_user.create_user()
        case "2":
            await delete_user.delete_user()
        case "3":
            await list_users.list_users()
        case "4":
            await change_pin.change_pin()
        case "5":
            await verify.verify_user()
        case "6":
            await get_logs.get_logs()
        case "7":
            await transfer.transfer()
        case "0":
            print("exit")
        case _:
            print("Invalid input")
            main()

def start():
    import asyncio

    asyncio.run(main())



def hashb(password):
    import bcrypt 
    password = str(password)
    bytes = password.encode('utf-8') 
    salt = bcrypt.gensalt() 
    hash = bcrypt.hashpw(bytes, salt) 
    return hash.decode('utf-8')

def matchpswd(password, hashed):
    import bcrypt
    password = str(password)
    hashed = str(hashed)
    lobytes = password.encode('utf-8')
    hashed = hashed.encode('utf-8')
    return bcrypt.checkpw(lobytes, hashed)

async def update_field(id, field, value):
    from surrealdb import Surreal
    async with Surreal("ws://localhost:8000/rpc") as db:
        await db.signin({"user": DBUSER, "pass": DBPSSWD})
        await db.use("user", "user")
        res = await db.query(f"UPDATE {id} SET {field} = '{value}'",{})
        return res


def get_time():
    import datetime
    return datetime.datetime.now().strftime("%m-%d-%y %H:%M:%S")