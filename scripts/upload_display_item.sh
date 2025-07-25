#!/bin/bash

# Check if directory argument is provided
if [ $# -ne 2 ]; then
    echo "Usage: $0 <directory> <upload_id>"
    exit 1
fi

DIRECTORY=$1

# Check if directory exists
if [ ! -d "$DIRECTORY" ]; then
    echo "Error: Directory $DIRECTORY does not exist"
    exit 1
fi

# Get canister ID from dfx.json
CANISTER_ID=$(dfx canister id constellation)
UPLOAD_ID=$2

# Check file sizes first
for file in "$DIRECTORY"/*; do
    if [ -f "$file" ]; then
        # Get file size in bytes
        size=$(stat -c %s "$file")
        # 2MB = 2097152 bytes
        if [ $size -gt 2097152 ]; then
            echo "Error: File $file is larger than 2MB"
            exit 1
        fi
    fi
done

# Loop through all files in directory
for file in "$DIRECTORY"/*; do
    if [ -f "$file" ]; then
        echo "Uploading file: $file"        
        
        # Stage the upload ID
        dfx canister call constellation stage_display_item "(\"$UPLOAD_ID\")"
        
        # Upload file using ic-file-uploader
        ic-file-uploader \
            $CANISTER_ID \
            "add_display_item" \
            $file \
            
        echo "Completed uploading: $file with ID: $UPLOAD_ID"
    fi
done

echo "All files uploaded successfully"
