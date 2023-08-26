from tkinter import messagebox
import os
import platform
import json


# base paths
PYTHON_FILES_FOLDER = os.path.dirname(os.path.abspath(__file__))
PROFILE_MANAGER_FOLDER = os.path.dirname(PYTHON_FILES_FOLDER)
WINDOWS_SCRIPTS_FOLDER = os.path.join(PROFILE_MANAGER_FOLDER, "WindowsScripts")

# run settings
RUN_SETTINGS_NAME = "run_settings.cfg"
DEF_RUN_SETTINGS_PATH = os.path.join(PROFILE_MANAGER_FOLDER, RUN_SETTINGS_NAME)

# last saved location of profiles txt file
PROGRAM_PATHS_NAME = "ProgramPaths.json"
PROGRAM_PATHS_PATH = os.path.join(PYTHON_FILES_FOLDER, PROGRAM_PATHS_NAME)

# profiles json
DEF_PROF_JSON_NAME = "MinecraftPlayProfiles.json"
DEF_PROF_JSON_PATH = os.path.join(PROFILE_MANAGER_FOLDER, DEF_PROF_JSON_NAME)

# Minecraft buttons folder
MC_BUTTONS_FOLDER_NAME = "MinecraftPlayButtons"
MC_BUTTONS_FOLDER_PATH = os.path.join(PROFILE_MANAGER_FOLDER, MC_BUTTONS_FOLDER_NAME)

# paths to scripts for running minecraft from ui
WINDOWS_RUN_BAT_PATH = os.path.join(WINDOWS_SCRIPTS_FOLDER, "RunMinecraft.bat")


def check_prof_file_exists():
    saved_path_file = PROGRAM_PATHS_PATH
    with open(saved_path_file, "r") as file:
        saved_path = json.load(file)["Profiles.json"]
    
    return os.path.exists(saved_path)

def get_prof_path(user_input: bool=True):
    # first try to read path from txt file in same folder
    saved_path_file = PROGRAM_PATHS_PATH
    with open(saved_path_file, "r") as file:
        saved_path = json.load(file)["Profiles.json"]
    
    if os.path.exists(saved_path):
        return saved_path
    else:
        if user_input:
            messagebox.showerror("Profiles JSON missing!", "The JSON file for profiles, at the last saved location, is missing!")
            user_try_default = messagebox.askyesno("Profiles JSON missing!", "Would you like to try the default location?")
        else:
            return ""
    
    # else if user specifies try default path
    if user_try_default:
        default_path = DEF_PROF_JSON_PATH
        if os.path.exists(default_path):
            set_new_prof_saved(default_path)
            return default_path
        else:
            messagebox.showerror("Profiles JSON missing!", "It appears it is also not at the default. Please choose an option in the menu")
    
    return ""

def set_new_prof_saved(new_path):
    saved_path_file = PROGRAM_PATHS_PATH
    
    # get old data
    with open(saved_path_file, "r") as file:
        data = json.load(file)
        data["Profiles.json"] = new_path

    # save new data
    with open(saved_path_file, "w") as file:
        json.dump(data, file, indent=4)

def get_default_minecraft_folder():
    '''Platform-independant'''
    if os.name == "posix":  # maxOS/Linux
        if platform.system() == 'Darwin':  # macOS
            minecraft_path = os.path.expanduser("~/Library/Application Support/minecraft")
        elif platform.system() == 'Linux':  # Linux
            minecraft_path = os.path.expanduser("~/.minecraft")
    elif os.name == 'nt':  # Windows
        minecraft_path = os.path.expanduser("~\\AppData\\Roaming\\.minecraft")
    else:
        raise NotImplementedError("Unsupported platform")

    return minecraft_path

def get_platform():
    if os.name == "posix":  # maxOS/Linux
        if platform.system() == 'Darwin':  # macOS
            return "mac"
        elif platform.system() == 'Linux':  # Linux
            return "linux"
    elif os.name == 'nt':  # Windows
        return "windows"
    else:
        raise NotImplementedError("Unsupported platform")

def get_default_minecraft_paths():
    mc_path = get_default_minecraft_folder()
    acc_path = os.path.join(mc_path, "launcher_accounts.json")
    options_path = os.path.join(mc_path, "options.txt")
    options_shaders_path = os.path.join(mc_path, "optionsshaders.txt")
    return mc_path, acc_path, options_path, options_shaders_path
