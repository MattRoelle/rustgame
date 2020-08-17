#/bin/bash
set -e

if [ -d "./resources" ]; then rm -rf ./resources; fi
mkdir resources

echo "Starting Build Script"
date

echo "Converting Aseprite (.ase) files"
(
	cd assets

	for f in *.ase*
	do
		output_file=$(echo $f | sed -e"s/\.aseprite//" | sed -e"s/\.ase//")
		echo "  $f -> resources/$output_file.png"
		aseprite -b "$f" --save-as "../resources/$output_file.png"
	done
)

echo "Copying tmx files"
cp -r ./assets/*.tmx ./resources/

echo "Done"
