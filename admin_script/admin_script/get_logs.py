import csv
import json
import os
from surrealdb import Surreal

from admin_script import main


async def get_logs():
    id = input("Enter Account ID: ")
    id = "user:" + id
    async with Surreal("wss://banking.saswdorf.de:8000/rpc") as db:
        await db.signin({"user": main.DBUSER, "pass": main.DBPSSWD})
        await db.use("user", "user")
        data = await db.select(id)
        logs = data["transactions"]
        logs = str.removeprefix(logs, "###")
        if(logs == ""):
            print("No logs found")
            return
        logs = logs.split("###")
        filename = str.removeprefix(data["id"], "user:") + '.csv'
        try:
            os.remove(filename)
        except OSError:
            pass        
        with open(filename, mode='w') as data_file:
            employee_writer = csv.writer(data_file, delimiter=',', quotechar='"', quoting=csv.QUOTE_MINIMAL)
            employee_writer.writerow(['Time', 'From', 'To', 'Amount'])
            for log in logs:
                log = json.loads(log)
                employee_writer.writerow([log["time"], log["from"], log["to"], log["amount"]])
