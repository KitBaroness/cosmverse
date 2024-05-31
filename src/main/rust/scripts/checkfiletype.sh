#!/bin/bash

checkFileType() {
    fileType=$1
    find ./ -type f -name "*.$fileType" -print -quit | grep -q .
    if [ $? -eq 0 ]; then
        echo "File type '$fileType' exists in the directory."
    else
        echo "No files of type '$fileType' found."
    fi
}

# Define an array of file types
declare -a fileTypes=("txt" "md" "log" "sh" "rs" "kt" "html" "css" "js" "png" "jpg" "xml")

# Loop through the array and check each file type
for fileType in "${fileTypes[@]}"; do
    checkFileType "$fileType"
done
