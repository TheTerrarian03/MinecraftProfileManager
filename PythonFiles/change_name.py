ACC_FILE = r"C:\Users\thete\AppData\Roaming\.minecraft\launcher_accounts.json"

# open file

file = open(ACC_FILE, "r")

new_data = ""

for line in file.readlines():
    if line.startswith("        \"name\""):
        new_line = "        \"name\" : \"nova\",\n"
    else:
        new_line = line

    new_data += new_line

file.close()

# write new data

file = open(ACC_FILE, "w")

file.write(new_data)

file.close()