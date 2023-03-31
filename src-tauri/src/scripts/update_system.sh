#!/bin/bash
#set -e

echo
tput setaf 3
echo "###############################################################################"
echo "#                         Tauri Application Updater                           #"
echo "#                                                                             #"
echo "#                                                                             #"
echo "#           This is simply a test script to simulate an actual update         #"
echo "#                script used in a full-fledge Tauri desktop app               #"
echo "#                                                                             #"
echo "###############################################################################"
tput sgr0
echo
echo "Hello $USER, Please Select What To Update."
echo
echo "########## Standard Users ##########"
echo
echo "1.  Arch & AUR Packages."
echo "2.  Flatpak Packages."
echo
echo
echo "Enter your selection, or just close window to exit..."
echo
echo

while :; do

read CHOICE

case $CHOICE in
    * )
      echo "#################################"
      echo "        Closing Terminal         "
      echo "#################################"
      sleep 2
      exit
      ;;
esac
done
