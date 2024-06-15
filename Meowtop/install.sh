echo "Welcome to Meowtop DE"

echo "Cleaning..."
sudo rm -rf /usr/bin/Meowtop
sudo rm -rf /usr/share/xsessions/Meowtop.desktop
sudo rm -rf /usr/bin/Meowtop
sudo rm -rf /usr/bin/meowtopde
sudo rm -rf /usr/libexec/meowtop-binary

echo "Installing files..."
sudo mkdir /usr/bin/Meowtop
sudo cp Meowtop.desktop /usr/share/xsessions/
sudo cp start-meowtop /usr/bin/Meowtop
sudo cp meowtopde /usr/bin
sudo cp meowtop-binary /usr/libexec/

echo "Running setup..."
sudo chmod +x /usr/bin/Meowtop/Meowtop
sudo chmod +x /usr/bin/meowtopde
sudo chmod +x /usr/libexec/meowtop-binary
