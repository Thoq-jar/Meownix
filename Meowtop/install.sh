echo "Welcome to Meowtop"

echo "Installing files..."
sudo mkdir /usr/bin/Meowtop
sudo cp Meowtop.desktop /usr/share/xsessions/
sudo cp Meowtop /usr/bin/Meowtop
sudo cp meowtop /usr/bin/local
sudo cp meowtop-binary /usr/libexec/

echo "Done! please logout and select Meowtop to complete installation"
sleep 44
