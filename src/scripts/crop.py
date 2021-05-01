from PIL import Image
import sys
import glob
import numpy as np

# Trim all png images with alpha in a folder
# Usage "python PNGAlphaTrim.py ../someFolder"

try:
    folderName = sys.argv[1]
except :
    print("Usage: python PNGPNGAlphaTrim.py ../someFolder")
    sys.exit(1)

filePaths = glob.glob(folderName + "**/*.png", recursive=True) #search for all png images in the folder

for filePath in filePaths:
    print(filePath)

    image=Image.open(filePath)
    imageBox = image.getbbox()
    cropped=image.crop(imageBox)
    cropped.save(filePath)