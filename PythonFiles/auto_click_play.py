import pyautogui
import time
import os
import sys

def get_button_image_paths(folder_path):
    image_paths = []

    for filename in os.listdir(folder_path):
        if filename.lower().endswith(('.png', '.jpg', '.jpeg', '.gif', '.bmp')):
            image_path = os.path.join(folder_path, filename)
            image_paths.append(image_path)

    return image_paths

def main(image_paths):
    # Find the coordinates of the green play button
    for image in image_paths:
        button_position = pyautogui.locateOnScreen(image, confidence=0.9)
        if button_position is not None:
            break

    if button_position is None:
        print("Button not found")
        return False

    # Calculate the center of the button
    button_x, button_y, button_width, button_height = button_position
    button_center_x = button_x + button_width // 2
    button_center_y = button_y + button_height // 2

    # Move the mouse to the button and click it
    pyautogui.moveTo(button_center_x, button_center_y)
    pyautogui.click()

    return True

if __name__ == '__main__':
    # play button images path
    try:
        CWD = sys.argv[1]
    except IndexError:
        CWD = "C:\\Users\\thete\\AppData\\Roaming\\.minecraft\\CUSTOM_SCRIPTS\\mc_play_buttons\\"
    
    try:
        MAX_TIME = int(sys.argv[2])
    except (IndexError, ValueError):
        MAX_TIME = 10
    
    print(MAX_TIME, CWD)

    # get image paths, saves exec time
    image_paths = get_button_image_paths(CWD)
    
    # Record the start time
    start_time = time.time()

    # main loop
    while True:
        # Check for time past MAX_TIME
        elapsed_time = time.time() - start_time
        if elapsed_time > MAX_TIME:
            print("Time limit exceeded")
            break

        # run main program, which will try to click the button
        success = main(image_paths)

        # if button found and clicked, break loop and end script
        if success:
            print("success")
            break
