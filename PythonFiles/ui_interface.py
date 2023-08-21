import tkinter as tk
from tkinter import ttk, messagebox, simpledialog, filedialog
from ProfileJSONManager import ProfileManager
from write_settings import write_settings
import DirectoryManager
import os
import subprocess
import json

class ProfileManagerApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Profile Manager")

        self.main_menu_frame = tk.Frame(self.root, relief=tk.RAISED, borderwidth=2)
        self.main_menu_frame.pack(padx=20, pady=20)

        self.create_main_menu_buttons()

        self.check_for_menu_errors()

        self.current_frame = self.main_menu_frame  # Set main menu as current frame

    def create_main_menu_buttons(self):
        button_labels = [
            ("Choose default profile", self.show_choose_default_frame),
            ("Edit Profiles", self.show_edit_profiles_frame),
            ("Write profile", self.write_profile),
            ("Run Minecraft", self.run_minecraft),
            ("Manage Program Dependencies", self.show_manage_dependencies_frame),
            ("Manage File Paths", self.show_manage_paths_frame),
            ("Exit Program", self.root.quit)
        ]

        self.main_menu_buttons = {}

        for label, command in button_labels:
            button = tk.Button(self.main_menu_frame, text=label, command=command)
            button.pack(fill=tk.X, padx=10, pady=5)
            self.main_menu_buttons[label] = button
        
        # print(self.main_menu_buttons)

    def show_frame(self, frame_to_show):
        if hasattr(self, 'current_frame'):
            self.current_frame.pack_forget()

        self.current_frame = frame_to_show
        self.current_frame.pack(padx=20, pady=20)

    def create_basic_frame(self, title, return_to_menu_func):
        frame = tk.Frame(self.root, relief=tk.RAISED, borderwidth=2)

        label = tk.Label(frame, text=title)
        label.pack(side=tk.TOP)  # Pack label at the top of the frame

        exit_button = tk.Button(frame, text="Exit", command=return_to_menu_func)
        exit_button.pack(side=tk.BOTTOM)  # Pack exit button at the bottom of the frame

        return frame

    def show_choose_default_frame(self):
        choose_default_frame = self.create_basic_frame("Choose Default Profile", self.show_main_menu_frame)
        
        profiles = self.get_existing_profiles()  # Replace with your code to get preset profiles
        self.selected_profile = tk.StringVar(value=self.get_default_profile() if profiles else "")

        profile_combobox = ttk.Combobox(choose_default_frame, textvariable=self.selected_profile, values=profiles)
        profile_combobox.pack()

        set_default_button = tk.Button(choose_default_frame, text="Set Default", command=lambda: self.set_default_profile(profile_combobox.get()))
        set_default_button.pack()

        self.show_frame(choose_default_frame)

    def show_edit_profiles_frame(self):
        edit_profiles_frame = self.create_basic_frame("Edit Profiles", self.show_main_menu_frame)
        
        edit_profiles = self.get_existing_profiles()  # Replace with your code to get preset profiles
        self.selected_edit_profile = tk.StringVar(value=self.get_default_profile() if edit_profiles else "")

        profile_combobox = ttk.Combobox(edit_profiles_frame, textvariable=self.selected_edit_profile, values=edit_profiles)
        profile_combobox.bind("<<ComboboxSelected>>", lambda event: self.load_profile(customization_dict))
        profile_combobox.pack()

        self.reset_profiles_list = lambda: profile_combobox.config(values=self.get_existing_profiles())

        load_profile_button = tk.Button(edit_profiles_frame, text="Load Profile")  # command set later in function, has to pass in a dict yet to be made
        load_profile_button.pack()

        new_profile_button = tk.Button(edit_profiles_frame, text="New Profile", command=self.new_profile)
        new_profile_button.pack()

        remove_profile_button = tk.Button(edit_profiles_frame, text="Remove Profile", command=self.remove_profile)
        remove_profile_button.pack()

        ttk.Separator(edit_profiles_frame, orient=tk.HORIZONTAL).pack(fill=tk.X, padx=10, pady=5)

        run_offline_var = tk.BooleanVar()
        run_offline_checkbutton = tk.Checkbutton(edit_profiles_frame, text="Run Minecraft Offline?", variable=run_offline_var)
        run_offline_checkbutton.pack()

        change_name_var = tk.BooleanVar()
        change_player_name_checkbutton = tk.Checkbutton(edit_profiles_frame, text="Change Player Name?", variable=change_name_var)
        change_player_name_checkbutton.pack()

        player_name_var = tk.StringVar()
        player_name_entry = tk.Entry(edit_profiles_frame, textvariable=player_name_var)
        player_name_entry.pack()

        auto_click_var = tk.BooleanVar()
        auto_click_play_checkbutton = tk.Checkbutton(edit_profiles_frame, text="Auto Click Play?", variable=auto_click_var)
        auto_click_play_checkbutton.pack()

        options_label = tk.Label(edit_profiles_frame, text="options.txt settings")
        options_label.pack()

        options_textbox = tk.Text(edit_profiles_frame, width=40, height=4)
        options_textbox.pack()

        options_shaders_label = tk.Label(edit_profiles_frame, text="optionsshaders.txt settings")
        options_shaders_label.pack()

        options_shaders_textbox = tk.Text(edit_profiles_frame, width=40, height=4)
        options_shaders_textbox.pack()

        # dictionary of functions to call when retrieving info is necessary
        customization_dict = {
            "offline_var": run_offline_var,
            "change_name_var": change_name_var,
            "new_name_var": player_name_var,
            "auto_click_var": auto_click_var,
            "options_textbox": options_textbox,
            "optionsshaders_textbox": options_shaders_textbox
        }

        save_button = tk.Button(edit_profiles_frame, text="Save Profile!", command=lambda: self.save_profile(customization_dict))
        save_button.pack()

        load_profile_button.config(command=lambda: self.load_profile(customization_dict))

        if self.selected_edit_profile:
            self.load_profile(customization_dict)

        self.show_frame(edit_profiles_frame)

    def write_profile(self):
        profile_name = ProfileManager(file_path=DirectoryManager.get_prof_path()).get_default_profile_name()
        print(f"Attempting to write settings for profile {profile_name}...")
        write_settings(DirectoryManager.get_prof_path(), profile_name)
        print(f"...Done!")

    def run_minecraft(self):
        platform = DirectoryManager.get_platform()
        if platform == "mac":
            raise NotImplementedError("Unsupported Platform")
        elif platform == "linux":
            raise NotImplementedError("Unsupported Platform")
        elif platform == "windows":
            subprocess.call(DirectoryManager.WINDOWS_RUN_BAT_PATH, shell=True)
        else:
            raise NotImplementedError("Unsupported Platform")

    def show_manage_dependencies_frame(self):
        manage_dependencies_frame = self.create_basic_frame("Manage Program Dependencies", self.show_main_menu_frame)
        self.show_frame(manage_dependencies_frame)

    def show_manage_paths_frame(self):
        manage_paths_frame = self.create_basic_frame("Manage File Paths", self.show_main_menu_frame)
        
        profiles_json_path = DirectoryManager.get_prof_path()
        if profiles_json_path == "": profiles_json_path = "Invalid- Please choose"

        profile_json_label = tk.Label(manage_paths_frame, text=profiles_json_path)
        profile_json_label.pack()

        profile_json_choose_default_button = tk.Button(manage_paths_frame, text="Set to Default", command=self.set_profile_json_to_default)
        profile_json_choose_default_button.pack(side=tk.LEFT)
        
        profile_json_choose_custom_buttom = tk.Button(manage_paths_frame, text="Set to Custom", command=self.set_profile_json_to_chosen)
        profile_json_choose_custom_buttom.pack(side=tk.LEFT)

        self.show_frame(manage_paths_frame)

    def show_main_menu_frame(self):
        self.check_for_menu_errors()
        self.show_frame(self.main_menu_frame)

    def check_for_menu_errors(self):
        # program deps
        
        # paths
        warnings = 0

        profiles_json_exists = DirectoryManager.check_prof_file_exists()
        if not profiles_json_exists:
            warnings += 1

        if warnings > 0:
            self.main_menu_buttons["Manage File Paths"].config(bg="red")
        else:
            self.main_menu_buttons["Manage File Paths"].config(bg="SystemButtonFace")

    # ----- PATH MANAGER FUNCTIONS -----

    def set_profile_json_to_default(self):
        DirectoryManager.set_new_prof_saved(DirectoryManager.DEF_PROF_JSON_PATH)
    
    def set_profile_json_to_chosen(self):
        return filedialog.askopenfilename(filetypes=[("JSON Files", "*.json")])

    # ----- CHOOSE DEFAULT PROFILE FRAME FUNCTIONS -----

    def get_existing_profiles(self):
        json_manager = ProfileManager(file_path=DirectoryManager.get_prof_path())
        return json_manager.get_profile_names()
    
    def get_default_profile(self):
        return ProfileManager(file_path=DirectoryManager.get_prof_path()).get_default_profile_name()

    def set_default_profile(self, profile_name):
        json_manager = ProfileManager(file_path=DirectoryManager.get_prof_path())
        json_manager.set_default_profile(profile_name)
        print(f"Set default profile to: {profile_name}")
    
    # ----- EDIT PROFILES FRAME FUNCTIONS -----

    def load_profile(self, customization_dict: dict):
        selected_profile = self.selected_edit_profile.get()
        if selected_profile:
            print(f"Loading profile: {selected_profile}")
            
            # get profile data from JSON
            profile_data = ProfileManager(DirectoryManager.get_prof_path()).get_data_for_profile(selected_profile)
            bat_options = profile_data["bat_options"]

            # batch file options
            customization_dict["offline_var"].set(bat_options["run_offline"])
            customization_dict["change_name_var"].set(bat_options["change_name"])
            customization_dict["new_name_var"].set(bat_options["new_name"])
            customization_dict["auto_click_var"].set(bat_options["auto_click_play"])

            # options.txt custom options
            options_str = ""
            for key, value in profile_data["options.txt"].items():
                new_str = str(key) + ":" + str(value) + "\n"
                options_str += new_str
            
            customization_dict["options_textbox"].delete(0.0,tk.END)
            customization_dict["options_textbox"].insert(tk.END,options_str)

            # optionsshaders.txt custom options
            options_shaders_str = ""
            for key, value in profile_data["optionsshaders.txt"].items():
                new_str = str(key) + "=" + str(value) + "\n"
                options_shaders_str += new_str
            
            customization_dict["optionsshaders_textbox"].delete(0.0,tk.END)
            customization_dict["optionsshaders_textbox"].insert(tk.END,options_shaders_str)
        else:
            messagebox.showwarning("Error", "Please select a profile to load.")

    def new_profile(self):
        # Ask for new profile name
        new_profile_name = simpledialog.askstring("New Profile", "Please enter a name for the new profile:")
        while not new_profile_name:
            messagebox.showwarning("New Profile", "Please enter a valid name!")
            new_profile_name = simpledialog.askstring("New Profile", "Please enter a name for the new profile:")

        # make new JSON profile
        json_handler = ProfileManager(DirectoryManager.get_prof_path())
        json_handler.update_profile(new_profile_name)

        self.selected_edit_profile.set(new_profile_name)

        # usually here I would load the default data for the new profile into the entries, but it would be redundant/silly

        # reload profiles list
        self.reset_profiles_list()

        print(f"Created new profile: {new_profile_name}")

    def remove_profile(self):
        selected_profile = self.selected_edit_profile.get()
        if selected_profile:
            is_user_sure = messagebox.askyesno("Remove Profile", f"Are you sure you want to remove {selected_profile}?")
            if is_user_sure:
                ProfileManager(DirectoryManager.get_prof_path()).delete_profile(selected_profile)
                self.reset_profiles_list()
                self.selected_edit_profile.set(self.get_existing_profiles()[0])
                print(f"Removed profile: {selected_profile}")
        else:
            messagebox.showwarning("Error", "Please select a profile to remove.")
    
    def save_profile(self, customization_dict: dict):
        selected_profile = self.selected_edit_profile.get()
        print(f"Saving profile {selected_profile}...")
        if selected_profile:
            # options.txt
            options_dict = {}
            for line in customization_dict["options_textbox"].get(0.0,tk.END).split("\n"):
                try:
                    key, value = line.split(":")
                    options_dict[key] = value
                except ValueError:
                    print("Error (non-critical): Invalid custom value in options.txt")
            
            # optionsshaders.txt
            options_shaders_dict = {}
            for line in customization_dict["optionsshaders_textbox"].get(0.0,tk.END).split("\n"):
                try:
                    key, value = line.split("=")
                    options_shaders_dict[key] = value
                except ValueError:
                    print("Error (non-critical): Invalid custom value in optionsshaders.txt")

            ProfileManager(DirectoryManager.get_prof_path()).update_profile(selected_profile,
                                                                            customization_dict["offline_var"].get(),
                                                                            customization_dict["change_name_var"].get(),
                                                                            customization_dict["new_name_var"].get(),
                                                                            customization_dict["auto_click_var"].get(),
                                                                            options_dict, options_shaders_dict)

            print("...Success!")
        else:
            messagebox.showwarning("Error", "Please select a profile to load.")

def main():
    root = tk.Tk()
    app = ProfileManagerApp(root)
    root.mainloop()

if __name__ == "__main__":
    main()
