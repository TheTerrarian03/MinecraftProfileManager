import sys
import ProfileJSONManager
import json


def write_settings(json_path, profile):
    json_handler = ProfileJSONManager.ProfileManager(json_path)

    if profile == None:
        profile = json_handler.get_profile_names()[0]
    
    try:
        # get data for profile
        profile_data = json_handler.get_data_for_profile(profile)
        
        # set bat ini file settings
        with open("bat_settings.ini", "w") as file:
            for key, value in profile_data["bat_options"].items():
                file.write(str(key) + "=" + str(value) + "\n")
        
        # set some variables for paths
        mc_path = json_path[:json_path.find(".minecraft")+11]
        acc_path = mc_path + "launcher_accounts.json"
        options_path = mc_path + "options.txt"
        options_shaders_path = mc_path + "optionsshaders.txt"

        # launcher_accounts.json setting
        # open file

        if profile_data["bat_options"]["change_name"] == True:
            data = json.load(acc_path)
            print(data)

            data["accounts"][0]["minecraftProfile"]["name"] = "banana"  # profile_data['bat_options']['new_name']
            print(data)

            json.dump(data, acc_path, indent=2)
            input()

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

    except Exception as e:
        print(e)

if __name__ == "__main__":
    try:
        PROFILES_JSON_PATH = sys.argv[1]
        PROFILE_NAME = sys.argv[2]
    except IndexError:
        PROFILES_JSON_PATH = ProfileJSONManager.get_default_path_to_json()
        PROFILE_NAME = ProfileJSONManager.ProfileManager(PROFILES_JSON_PATH).get_default_profile_name()

        print(PROFILES_JSON_PATH, PROFILE_NAME)

    write_settings(PROFILES_JSON_PATH, PROFILE_NAME)