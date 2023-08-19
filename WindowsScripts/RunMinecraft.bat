@echo off
setlocal

REM Check if Python 3 is installed
python --version 2>NUL
if %errorlevel% neq 0 (
    echo Python 3 is not installed on this system.
    echo Asking user for permission to continue
    
    REM Launch graphical message box
    cscript //nologo ask_install_python.vbs "Python 3 is not installed on this system, and is required. Do you want the link to install Python to be opened for you?"

    cscript //nologo send_message.vbs "Please re-run this program once Python has been successfully installed. Thanks!"

    exit
)

set TRUE=1==1
set FALSE=1==0

: write settings for profile
start /wait python3 ..\PythonFiles\write_settings.py

: assign settings file
set SETTINGS_FILE=..\run_settings.ini

: getting bat settings
for /f "tokens=1,2 delims==" %%a in (%SETTINGS_FILE%) do (
    : disabling/enabling wifi
    if %%a==run_offline if %%b==True set OFFLINE=%TRUE%
    if %%a==run_offline if %%b==False set OFFLINE=%FALSE%

    : setting name
    if %%a==change_name if %%b==True set SETNAME=%TRUE%
    if %%a==change_name if %%b==False set SETNAME=%FALSE%

    : new name
    if %%a==new_name set NEWNAME=%%b

    : auto click play button
    if %%a==auto_click_play if %%b==True set AUTOCLICK=%TRUE%
    if %%a==auto_click_play if %%b==False set AUTOCLICK=%FALSE%
)

: if working with wifi, ask for admin access
if %OFFLINE% (
        if not "%1"=="am_admin" (
        powershell -Command "Start-Process -Verb RunAs -FilePath '%0' -ArgumentList 'am_admin'"
        exit /b 0
    )
)

: now, disable wifi if needed
if %OFFLINE% (
    netsh interface set interface "Wi-Fi" Disable
    netsh interface set interface "Ethernet" Disable
)

: run minecraft
python -c "import subprocess; subprocess.Popen(['C:\Program Files (x86)\Minecraft Launcher\MinecraftLauncher.exe'])"

: run press button
if %AUTOCLICK% (
    : run python script to look for button to press
    python3 ..\PythonFiles\auto_click_play.py ..\mc_play_buttons\ 30
) else (
    : pause and ask user to press enter when minecraft played
    cscript //nologo send_message.vbs "Please press ENTER or RETURN here once you have pressed the PLAY button."
)

if %OFFLINE% (
    netsh interface set interface "Wi-Fi" Enable
    netsh interface set interface "Ethernet" Enable
)

endlocal

exit /b 0
