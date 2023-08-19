Set objArgs = WScript.Arguments
messageText = objArgs(0)
response = MsgBox(messageText, vbYesNo + vbExclamation, "Minecraft Profile Manager")

If response = vbYes Then
    Set objShell = CreateObject("WScript.Shell")
    objShell.Run "https://www.python.org/downloads/", 1, True
End If
