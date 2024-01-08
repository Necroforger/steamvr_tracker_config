# Steamvr Tracker Configuration
A small tool that makes the workflow of configuring your devices as trackers slightly more convenient.

## Screenshots
![img](screenshots/screenshot1.png)

## How do
Download the executable from releases. Place it in it's own folder so the folders and backups it creates don't pollute your desktop.

1. Disconnect all steamvr devices. E.g headsets, base stations, and controllers.
2. Launch the executable.
3. Click search until a device is successfully discovered.
4. Select the model you would like to use with your device. ( Tracker is recommended for displaying battery life)
5. 'Configure as Tracker'

## How undo
When you configure a device as a tracker for the first time, a backup of the stock configuration will be saved. Pressing the `restore` button will upload the original configuration.

## Considerations
- If your steamvr exists in a location other than `C:\Program Files (x86)\Steam\steamapps\common\SteamVR\tools\lighthouse\bin\win64\lighthouse_console.exe` you will have to manually enter the path of the lighthouse_console.exe executable.
- Try not to lose your stock configuration.

## Build instructions
- `pnpm run tauri build`
- dependencies:
    - rust nightly
    - pnpm