# SRXDUnityPlayerGuard
A simple application that makes sure that the `Mono` version of `UnityPlayer.dll` is always used, especially after game updates.

## Installation
1. First drop `SRXDUnityPlayerGuard.exe`(Windows) or `SRXDUnityPlayerGuard`(Linux) into your `Spin Rhythm` game folder (by default it is `C:\Program Files\Steam\steamapps\common\Spin Rhythm`).
2. Add the following line into your Steam launch options for Spin Rhythm XD.

### For Windows
```bash
"C:\Windows\System32\cmd.exe" /c .\SRXDUnityPlayerGuard.exe & start "" %command%
```
### For Linux
```bash
./SRXDUnityPlayerGuard ; %command%
```

## Note
If you've already got your backup `UnityPlayer.dll` under a name that isn't `UnityPlayer.bak.dll`, please either
- use the `--backuppath <filename here>` argument. For example on Windows: 
```bash
"C:\Windows\System32\cmd.exe" /c .\SRXDUnityPlayerGuard.exe --backuppath "BackupOfUnityPlayer.dll" & start "" %command%
```
- Or rename it to `UnityPlayer.bak.dll`.

If you used SRXDBepInExInstaller to install BepInEx, you don't need to worry about this.

## How?
The application makes sure that `UnityPlayer_Mono.dll` has the same hash as `UnityPlayer.dll`. 
Otherwise `UnityPlayer.dll` is renamed to `UnityPlayer.bak.dll`, and `UnityPlayer_Mono.dll` is copied to `UnityPlayer.dll`.
