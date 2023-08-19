import tkinter as tk
from tkinter import ttk
import sys
import ProfileJSONManager
import write_settings


def CLEAN_DATA(data: str):
    # check for list
    if data.startswith("[") and data.endswith("]"):
        data = data.replace("\"","'")
        data = data.replace(", ", ",")
    
    return data

class ComplexInterface:
    def __init__(self, root, profiles_json_path):
        self.root = root
        self.root.title("Complex Tkinter Interface")
        
        # Top Frame
        top_frame = tk.Frame(root)
        top_frame.pack(fill="both", expand=True)
        
        # Profile Choose Side
        profile_choose_frame = tk.Frame(top_frame)
        profile_choose_frame.pack(side="left", padx=10, pady=10, fill="y")
        
        tk.Label(profile_choose_frame, text="Profile", font=("Helvetica", 16, "bold")).pack(anchor="w")
        
        self.json_handler = ProfileJSONManager.ProfileManager(profiles_json_path)
        profile_names = self.json_handler.get_profile_names()
        
        self.profile_var = tk.StringVar()
        self.profile_var.set(profile_names[0])

        ttk.Combobox(profile_choose_frame, textvariable=self.profile_var, values=profile_names).pack(fill="x")
                
        # Profile Settings Side
        profile_settings_frame = tk.Frame(top_frame)
        profile_settings_frame.pack(side="left", padx=10, pady=10, fill="both", expand=True)
        
        tk.Label(profile_settings_frame, text="Profile Settings", font=("Helvetica", 16, "bold")).pack(anchor="w")

        tk.Button(profile_choose_frame, text="Load Profile", command=self.load_profile).pack(padx=10)
        tk.Button(profile_choose_frame, text="Remove Profile", command=self.remove_profile).pack(padx=10)
        tk.Button(profile_choose_frame, text="Add Profile", command=self.add_profile).pack(padx=10)
        
        self.disable_wifi_var = tk.BooleanVar()
        self.change_name_var = tk.BooleanVar()
        self.new_name_var = tk.StringVar()

        tk.Checkbutton(profile_settings_frame, text="Disable/Enable WiFi", variable=self.disable_wifi_var).pack(anchor="w")
        tk.Checkbutton(profile_settings_frame, text="Change Player Name", variable=self.change_name_var).pack(anchor="w")
        tk.Entry(profile_settings_frame, textvariable=self.new_name_var).pack(fill="x", pady=5)
        
        tk.Button(profile_settings_frame, text="Save Profile!", command=self.save_profile).pack(padx=10)
        tk.Button(profile_settings_frame, text="Write Settings!", command=self.write_settings).pack(padx=10)

        # Minecraft Settings Side

        minecraft_settings_frame = tk.Frame(top_frame)
        minecraft_settings_frame.pack(side="right", padx=10, pady=10, fill="both", expand=True)

        tk.Label(minecraft_settings_frame, text="Minecraft Settings", font=("Helvetica", 16, "bold")).pack(anchor="w")

        tk.Label(minecraft_settings_frame, text="options.txt custom options").pack()

        self.options_box = tk.Text(minecraft_settings_frame, )
        self.options_box.pack(padx=10, pady=10, fill="both", expand=True)
        
        tk.Label(minecraft_settings_frame, text="optionsshaders.txt custom options").pack()

        self.options_shaders_box = tk.Text(minecraft_settings_frame, wrap=tk.WORD)
        self.options_shaders_box.pack(padx=10, pady=10, fill="both", expand=True)

        # load first profile
        self.load_profile()

    def load_profile(self):
        # load and get data
        profile_data = self.json_handler.get_data_for_profile(self.profile_var.get())
        bat_options = profile_data["bat_options"]

        # batch file options
        self.disable_wifi_var.set(bat_options["disable_wifi"])
        self.change_name_var.set(bat_options["change_name"])
        self.new_name_var.set(bat_options["new_name"])

        # options.txt custom options
        options_str = ""
        for key, value in profile_data["options.txt"].items():
            new_str = str(key) + ":" + str(value) + "\n"
            options_str += new_str
        
        self.options_box.delete(0.0,tk.END)
        self.options_box.insert(tk.END,options_str)

        # optionsshaders.txt custom options
        options_shaders_str = ""
        for key, value in profile_data["optionsshaders.txt"].items():
            new_str = str(key) + "=" + str(value) + "\n"
            options_shaders_str += new_str
        
        self.options_shaders_box.delete(0.0,tk.END)
        self.options_shaders_box.insert(tk.END,options_shaders_str)

    def remove_profile(self):
        print("removing profile")
    
    def add_profile(self):
        print("adding profile")
    
    def save_profile(self):
        # options.txt
        options_dict = {}
        for line in self.options_box.get(0.0,tk.END).split("\n"):
            try:
                key, value = line.split(":")
                options_dict[key] = CLEAN_DATA(value)
            except ValueError:
                print("Error: Invalid custom value in options.txt")
        
        options_shaders_dict = {}
        for line in self.options_shaders_box.get(0.0,tk.END).split("\n"):
            try:
                key, value = line.split("=")
                options_shaders_dict[key] = value
            except ValueError:
                print("Error: Invalid custom value in optionsshaders.txt")

        self.json_handler.update_profile(self.profile_var.get(), self.disable_wifi_var.get(), self.change_name_var.get(), self.new_name_var.get(), options_dict, options_shaders_dict)
    
    def write_settings(self):
        write_settings.write_settings(PROFILES_JSON_PATH, self.profile_var.get())

if __name__ == "__main__":
    try:
        PROFILES_JSON_PATH = sys.argv[1]
    except IndexError:
        PROFILES_JSON_PATH = ProfileJSONManager.get_default_path_to_json()

    root = tk.Tk()
    app = ComplexInterface(root, PROFILES_JSON_PATH)
    root.mainloop()
