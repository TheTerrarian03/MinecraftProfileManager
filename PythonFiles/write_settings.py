import sys
import ProfileJSONManager
import json
import DirectoryManager
import os


def write_settings(json_path, profile):
    json_handler = ProfileJSONManager.ProfileManager(json_path)

    if profile == None:
        profile = json_handler.get_default_profile_name()
    
    # try:
    # get data for profile
    profile_data = json_handler.get_data_for_profile(profile)
    
    # set bat ini file settings
    with open(DirectoryManager.DEF_RUN_SETTINGS_PATH, "w") as file:
        for key, value in profile_data["bat_options"].items():
            file.write(str(key) + "=" + str(value) + "\n")
    
    # set some variables for paths
    mc_path, acc_path, options_path, options_shaders_path = DirectoryManager.get_default_minecraft_paths()

    # launcher_accounts.json setting
    # open file

    if profile_data["bat_options"]["change_name"] == True:
        # load data in the launcher_accounts.json file
        data = json.load(open(acc_path, "r"))

        # add ids to a list for later use
        account_ids = []
        for account_key, account_data in data["accounts"].items():
            account_ids.append(account_key)

        # change name for each id to the new desired name
        for account_id in account_ids:
            data["accounts"][account_id]["minecraftProfile"]["name"] = "banana"

        # write new and updated data back to the launcher_accounts.json file
        json.dump(data, open(acc_path, "w"), indent=2)

    # options.txt setting
    options_data = ""
    with open(options_path, "r") as file:
        for line in file.readlines():
            overwritten = False
            for cust_option, cust_setting in profile_data["options.txt"].items():
                if line.startswith(cust_option):
                    options_data += str(cust_option) + ":" + str(cust_setting) + "\n"
                    overwritten = True
                    break
            if not overwritten:
                options_data += line
    
    with open(options_path, "w") as file:
        file.write(options_data)
    
    # optionsshaders
    optionsshaders_data = ""
    with open(options_shaders_path, "r") as file:
        for line in file.readlines():
            overwritten = False
            for cust_option, cust_setting in profile_data["optionsshaders.txt"].items():
                if line.startswith(cust_option):
                    optionsshaders_data += str(cust_option) + "=" + str(cust_setting) + "\n"
                    overwritten = True
                    break
            if not overwritten:
                optionsshaders_data += line
    
    with open(options_shaders_path, "w") as file:
        file.write(optionsshaders_data)

        # launcher_accounts.json

    # except Exception as e:
    #     print(e)

if __name__ == "__main__":
    try:
        PROFILES_JSON_PATH = sys.argv[1]
        PROFILE_NAME = sys.argv[2]
    except IndexError:
        PROFILES_JSON_PATH = DirectoryManager.get_prof_path()
        PROFILE_NAME = ProfileJSONManager.ProfileManager(PROFILES_JSON_PATH).get_default_profile_name()

        print(PROFILES_JSON_PATH, PROFILE_NAME)

    write_settings(PROFILES_JSON_PATH, PROFILE_NAME)
