import asyncio
import decimal
import json

from surrealdb import Surreal

from admin_script import main
async def transfer():
    sender = input("From: ")
    reciever = input("To: ")
    amount = input("Amount: ")
    async with Surreal("wss://banking.saswdorf.de:8000/rpc") as db:
        await db.signin({"user": main.DBUSER, "pass": main.DBPSSWD})
        await db.use("user", "user")
        sender = await db.select("user:" + sender)
        reciever = await db.select("user:" + reciever)
        bank = await db.select("user:zentralbank")
        if decimal.Decimal(sender["balance"]) < decimal.Decimal(amount):
            red = '\033[91m'
            end = '\033[0m'
            print(f"\n{red}Insufficient balance\n{end}")
            return
        await main.update_field(sender["id"], "balance", str(decimal.Decimal(sender["balance"]) - decimal.Decimal(amount)))
        amount_with_tax = decimal.Decimal(amount) - decimal.Decimal(amount) * decimal.Decimal("0.1")
        amount_for_bank = decimal.Decimal(amount) * decimal.Decimal("0.1")
        await main.update_field(reciever["id"], "balance", str(decimal.Decimal(reciever["balance"]) + amount_with_tax))
        await main.update_field(bank["id"], "balance", str(decimal.Decimal(bank["balance"]) + amount_for_bank))
        logSender = {
            "time" : str(main.get_time()),
            "from" : sender["name"],
            "to" : reciever["name"],
            "amount" : str(amount),
        }
        logReciever = {
            "time" : str(main.get_time()),
            "from" : sender["name"],
            "to" : reciever["name"],
            "amount" : str(amount_with_tax),
        }
        logBank = {
            "time" : str(main.get_time()),
            "from" : sender["name"],
            "to" : reciever["name"],
            "amount" : str(amount_for_bank),
        }
        logSender = json.dumps(logSender)
        logReciever = json.dumps(logReciever)
        logBank = json.dumps(logBank)
        async with asyncio.TaskGroup() as tg:
            task1 = tg.create_task(main.update_field(sender["id"], "transactions", sender["transactions"] + "###" + logSender))
            task2 = tg.create_task(main.update_field(reciever["id"], "transactions", reciever["transactions"] + "###" + logReciever))
            task3 = tg.create_task(main.update_field(bank["id"], "transactions", bank["transactions"] + "###" + logBank))

        print(f"\n\033[92mTransfer successful\n\033[0m")
        

