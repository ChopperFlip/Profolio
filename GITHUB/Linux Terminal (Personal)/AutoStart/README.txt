Title: Disable Autostart Applications via Bash Terminal
Author: Phillip Howard

Description: Safely disable specific graphical applications from launching automatically at desktop sesssions.
             It follows the XDG Autostart Specification by overriding system-wide .desktop files with user-level configurations

             Intended for Fedora Linux 42 KDE Plasma and to save RAM
             NOTE: I PERSONALLY UPDATE REGULARLY AND DO NOT NEED COMPUTER CALENDER. This saved me 1GB of RAM

What: 1. Searches for .desktop files in /etc/xdg/autostart/
      2. Copies selected .desktop files into ~/.config/autostart/
      3. Appends Hidden=True to those files to disable them for the current user only
      4. Preserves the system-wide files

Usage: Open terminal and copy past the commands

Contributing:  If you'd like to contribute, please fork the repository and open a pull request to the `main` branch.
