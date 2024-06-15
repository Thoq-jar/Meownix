echo "Welcome to Meowtop"

echo "Installing files..."
sudo rm -rf /usr/bin/Meowtop
sudo rm -rf /usr/share/xsessions/Meowtop.desktop
sudo rm -rf /usr/bin/start-meowtop
sudo rm -rf /usr/libexec/meowtop-binary
sudo rm -rf 

sudo mkdir /usr/bin/Meowtop
sudo cp Meowtop.desktop /usr/share/xsessions/
sudo cp Meowtop /usr/bin/Meowtop
sudo cp start-meowtop /usr/bin/
sudo cp meowtop-binary /usr/libexec/

echo "Done! please logout and select Meowtop in Login Manager to complete installation"
