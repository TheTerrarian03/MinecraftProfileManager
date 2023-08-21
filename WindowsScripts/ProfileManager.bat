@echo off
setlocal

REM Check if Python 3 is installed
python --version 2>NUL
if %errorlevel% neq 0 (
    REM Launch graphical message box
    cscript //nologo ask_install_python.vbs "Python 3 is not installed on this system, and is required. Do you want the link to install Python to be opened for you?"

    cscript //nologo send_message.vbs "Please re-run this program once Python has been successfully installed. Thanks!"

    exit
)

endlocal

%~dp0..\PythonFiles\ui_interface.py
pause