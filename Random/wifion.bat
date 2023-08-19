if not "%1"=="am_admin" (
    powershell -Command "Start-Process -Verb RunAs -FilePath '%0' -ArgumentList 'am_admin'"
    exit /b
)

netsh interface set interface "Wi-Fi" Enable
netsh interface set interface "Ethernet" Enable