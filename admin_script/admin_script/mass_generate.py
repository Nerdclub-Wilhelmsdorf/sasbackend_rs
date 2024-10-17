from admin_script import create_user
import os
import csv
import qrcode
async def mass_generate():
    student_numbers = ["203782","203876","203780"]
    ids = []
    #create folder for mass_gen, delete if exists
    if os.path.exists("mass_gen"):
        os.system("rm -r mass_gen")
    os.system("mkdir mass_gen")
    os.chdir("mass_gen")
    os.mkdir("qr_codes")
    csv_file = open("users.csv", "w")
    csv_writer = csv.writer(csv_file)
    csv_writer.writerow(["ID", "Student Number", "Pin"])
    for student_number in student_numbers:
        id = create_user.random_string(12)
        while ids.__contains__(id):
            id = create_user.random_string(12)
        ids.append(id)
        pin = create_user.random4int()
        await create_user.new_user(id, student_number, "0", pin, False)
        csv_writer.writerow([id, student_number, pin])
        import qrcode
        img = qrcode.make("w:" + id)
        #create file, then write qr to it
        img.save("qr_codes/" + student_number + ".png")
    csv_file.close()
