@echo off

:: Check for admin access and request if not already elevated
net session >nul 2>&1
if %errorLevel% == 0 (
    echo Admin access granted.
) else (
    echo Requesting admin access...
    powershell -Command "Start-Process -Verb RunAs -FilePath '%0' -ArgumentList 'am_admin'"
    exit /b 0
)

:: Turn Wi-Fi on or off based on the second argument
if "%~2"=="true" (
    netsh interface set interface "Wi-Fi" Disable
    netsh interface set interface "Ethernet" Disable
    echo Wi-Fi is now OFF.
) else (
    echo Invalid argument for Wi-Fi state. Use "true" or "false" as the second argument.
    goto :eof
)

:: Check if an executable path is provided as the first argument
if "%~1"=="" (
    echo Please provide the path to the executable as the first argument.
    goto :eof
)

:: Run the executable
start "" "%~1"

:: Wait for the executable to finish (optional)
:: You can add a timeout or any other logic here if needed
:: Example: ping 127.0.0.1 -n 5 > nul

pause

:: Turn Wi-Fi on or off based on the second argument
if "%~2"=="true" (
    netsh interface set interface "Wi-Fi" Enable
    netsh interface set interface "Ethernet" Enable
    echo Wi-Fi is now ON.
) else (
    echo Invalid argument for Wi-Fi state. Use "true" or "false" as the second argument.
    goto :eof
)