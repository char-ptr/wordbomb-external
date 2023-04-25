from time import sleep
import cv2
import numpy as np
import pytesseract 
import os
import pygetwindow as gw
import PIL
from PIL import Image
import PIL.ImageGrab
import PIL.ImageEnhance
import PIL.ImageOps


def crop(image):

    # Load image, grayscale, Gaussian blur, Otsu's threshold
    # original = image.copy()
    guh = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)

    lower_blue = np.array([30,33,75]) 
    upper_blue = np.array([33,36,77]) 

    thresh = cv2.inRange(guh, lower_blue, upper_blue)
    # Vizualize the mask
    # thresh = cv2.bitwise_not(threshb)

    # thresh = cv2.threshold(b_component, 0, 255, cv2.THRESH_BINARY+cv2.THRESH_OTSU)[1]

    # Perform morph operations, first open to remove noise, then close to combine
    noise_kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (3,3))
    opening = cv2.morphologyEx(thresh, cv2.MORPH_OPEN, noise_kernel, iterations=1)
    # close_kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (7,7))
    # close = cv2.morphologyEx(opening, cv2.MORPH_CLOSE, close_kernel, iterations=3)

    # Find enclosing boundingbox and crop ROI
    coords = cv2.findNonZero(opening)
    x,y,w,h = cv2.boundingRect(coords)
    cv2.rectangle(image, (x, y), (x + w, y + h), (36,255,12), 2)
    crop = thresh[y - 10:y+h + 20, x-10:x+w+20]
    # cv2.imshow('crop', crop)
    cv2.imwrite('crop.png', crop)
    return Image.fromarray(crop)

# setup tess

scp = os.environ["SCOOP"]
scpath = scp or "C:\\users\\{name}\\scoop"
pytesseract.pytesseract.tesseract_cmd = rf'{scpath}\apps\tesseract\current\tesseract.exe'

wind = gw.getWindowsWithTitle('Roblox')[0]

x, y = wind.topleft
w, h = wind.size
w = w - 330 # 330 is the width of the chat box
c,c2 = x + ((w/2)- 270), y + ((h/2) - 250)
img = PIL.ImageGrab.grab(bbox=(c,c2 , c + 570, c2 + 500)).convert('RGB')

# img.save('screenshot.png')
enhancer = PIL.ImageEnhance.Contrast(img)
image = enhancer.enhance(2)
image = PIL.ImageOps.invert(image)
image = image.point(lambda x: 0 if x < 10 else 255)
image = image.convert('L')
other = pytesseract.image_to_string(image,config="--oem 3 --psm 6").lower()

res = ""
def in_range(a,n,n2):
    return n <= a <= n2
def in_range2m(a,n):
    return in_range(a,n-2,n+2)
if "quick" in other:
# if "type" in other:

    opencv_ready = cv2.cvtColor(np.array(img), cv2.COLOR_RGB2BGR)

    prompt = pytesseract.image_to_string(crop(opencv_ready),config="-c tessedit_char_whitelist=ABCDEFGHIJKLMNOPQRSTUVWXYZ --oem 3 --psm 7").strip().replace("1","I").replace("5","s").replace("0","o").replace("PL","il").replace("|","i").replace(" ", "").replace("\n", "").upper()

    print(f"1 {prompt}")
elif "click" in other:
    print(f"2 {x} {y} {w} {h}")
else :
    print("0")
# cv2.waitKey(500)
# 