from tkinter import messagebox
import os
import platform


# run settings
RUN_SETTINGS_NAME = "run_settings.cfg"
DEF_RUN_SETTINGS_PATH = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), RUN_SETTINGS_NAME)

# last saved location of profiles txt file
SAVED_PROFILES_PATH_NAME = "ProgramPaths.txt"
DEF_PROFILES_FILE_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), SAVED_PROFILES_PATH_NAME)

# profiles json
DEF_PROF_JSON_NAME = "MinecraftPlayProfiles.json"
DEF_PROF_JSON_PATH = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), DEF_PROF_JSON_NAME)


class LastSavedPathLoc:
    def check_exists(self):
        saved_path_file = DEF_PROFILES_FILE_PATH
        with open(saved_path_file, "r") as file:
            line = file.readline()
            saved_path = line
        
        return os.path.exists(saved_path)

    def get_path(self, user_input: bool=True):
        # first try to read path from txt file in same folder
        saved_path_file = DEF_PROFILES_FILE_PATH
        with open(saved_path_file, "r") as file:
            line = file.readline()
            saved_path = line
        
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
                self.set_new_saved(default_path)
                return default_path
            else:
                messagebox.showerror("Profiles JSON missing!", "It appears it is also not at the default. Please choose an option in the menu")
        
        return ""

    def set_new_saved(self, new_path):
        saved_path_file = DEF_PROFILES_FILE_PATH
        with open(saved_path_file, "w") as file:
            file.write(new_path)

class ProfileJSON:
    def get_default_minecraft_folder(self):
        '''Platform-independant'''
        if os.name == "posix":  # maxOS/Linux
            if platform.system() == 'Darwin':  # macOS
                minecraft_path = os.path.expanduser("~/Library/Application Support/minecraft")
            elif platform.system() == 'Linux':  # Linux
                minecraft_path = os.path.expanduser("~/.minecraft")
        elif os.name == 'nt':  # Windows
            path = os.path.expanduser("~\\AppData\\Roaming\\.minecraft")
        else:
            raise NotImplementedError("Unsupported platform")

        return path
