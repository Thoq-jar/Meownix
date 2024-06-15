echo "Welcome to Prism 3"
echo "Developed by Thoq"

echo "Installing files..."
sudo mkdir /usr/bin/Prism
sudo cp Prism.desktop /usr/share/xsessions/
sudo cp Prism /usr/bin/Prism
sudo cp start-prismx3 /usr/bin/
sudo cp prism-binary /usr/libexec/

echo "Done! please logout and select Prism 3 to complete installation"
sleep 44
