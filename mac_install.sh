upath=$(eval echo ~$USER)
active_shell=$(eval echo "$SHELL")
installed=0
sleep 1
function cleanup()
{
    if [ $installed -eq 0 ]; then
        echo ""
        echo "Cleaning up..."
        echo ""
        echo "Installation aborted."
        rm -rf $upath/.lia
    fi
}
if [ -d "$upath/.lia" ]; then
    echo "Lia is already installed in your home directory"
    read -p "Would you like to continue and remove old files? (y/n) " -n 1
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Removing old files"
        rm -rf $upath/.lia
    else
        echo "Installation aborted"
        exit
    fi
    
fi
trap cleanup EXIT
echo "Welcome to Lia Installer on MacOS"
echo "---"
echo "Requirements:"
echo "  - Curl"
echo "  - Git (optional)"
read -p "   Would you like to continue? (y/n) " -n 1
#[ "$UID" -eq 0 ] || exec sudo bash "$0" "$@"
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    stype=0
    if [[ "$active_shell" = "/bin/zsh" ]]; then
       stype=1
    elif [[ "$active_shell" = "/bin/bash" ]]; then
       stype=2
    else
       echo "Unsupported shell, exiting..."
       exit 1
    fi
    mkdir $upath/.lia
    mkdir $upath/.lia/bin
    echo "Downloading Lia..."
    INSTANCE_REGION=$(curl -s 'https://lia-server.herokuapp.com/getLatestLia')
    URL=$(curl -s "https://ecjdmzrdopsfaqxxtoga.supabase.co/storage/v1/object/public/releases/lia/"$INSTANCE_REGION"/lia" --output $upath/.lia/bin/lia)
    echo "Downloading Lia... Done"
    echo "Installing Lia..."
    chmod 111 $upath/.lia/bin/lia
    if [ $stype = 1 ]; then
        echo "" >> $upath/.zshrc
        echo "# Lia Bin" >> $upath/.zshrc
        echo "export PATH=$upath/.lia/bin:\$PATH" >> $upath/.zshrc
    elif [ $stype = 2 ]; then
        echo "" >> $upath/.zshrc
        echo "# Lia Bin" >> ~/.bashrc
        echo "export PATH=$PATH:~/.lia/bin" >> ~/.bashrc
    fi
    echo "Installing Lia... Done"
    if [ $stype = 1 ]; then
        echo "To set environment variables, run 'source ~/.zshrc'"
    elif [ $stype = 2 ]; then
        echo "To set environment variables, run 'source ~/.bashrc'"
    fi
    echo "---"
    echo "Lia is installed!"
    installed=1
fi