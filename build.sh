#/bin/bash

if [ -d "./resources" ]; then rm -rf ./resources; fi
mkdir resources

echo "Starting Build Script"
date

echo "Converting Aseprite (.ase) files"
for f in $(ls assets)
do
	output_file=$(echo $f | sed -e"s/\.ase//")
    echo "  $f -> resources/$output_file.png"
	aseprite -b "assets/$f" --save-as "resources/$output_file.png"
done

echo "Done"
