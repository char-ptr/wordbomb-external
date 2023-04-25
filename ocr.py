
import pytesseract 
import PIL
import PIL.ImageGrab
import PIL.ImageEnhance
import PIL.ImageOps
import pygetwindow as gw
import os;

name = os.getlogin()
scp = os.environ["SCOOP"]
scpath = scp or "C:\\users\\{name}\\scoop"
pytesseract.pytesseract.tesseract_cmd = rf'{scpath}\apps\tesseract\current\tesseract.exe'
wind = gw.getWindowsWithTitle('Roblox')[0]
x, y = wind.topleft
w, h = wind.size
w = w - 330 # 330 is the width of the chat box
c,c2 = x + ((w/2)- 270), y + ((h/2) - 250)
img = PIL.ImageGrab.grab(bbox=(c,c2 , c + 570, c2 + 500))
# img.save('screenshot.png')
enhancer = PIL.ImageEnhance.Contrast(img)
image = enhancer.enhance(2)
image = PIL.ImageOps.invert(image)
image = image.point(lambda x: 0 if x < 10 else 255)
image = image.convert('L')
other = pytesseract.image_to_string(image,config="--oem 3 --psm 6").lower()

# other = "click"
res = ""
def in_range(a,n,n2):
    return n <= a <= n2
def in_range2m(a,n):
    return in_range(a,n-2,n+2)
if "quick" in other:

    imgd = img.getdata()
    nimgd = []
    for data in imgd:
        xm = abs(32 - data[0])
        ym = abs(35 - data[1])
        zm = abs(77 - data[2])
        if in_range2m(data[0],32) and in_range2m(data[1],35) and in_range2m(data[2],77):
            nimgd.append((255 - xm, 255 - ym, 255 - zm, 255))
        else:
            nimgd.append((0, 0, 0, 255))

    img.putdata(nimgd)

    enhancer = PIL.ImageEnhance.Contrast(img)
    image = enhancer.enhance(2)
    image = PIL.ImageOps.invert(image)
    image = image.point(lambda x: 0 if x < 10 else 255)
    image = image.convert('L')
    image.save('screenshot3.png')
    prompt = pytesseract.image_to_string(image,config="-c tessedit_char_whitelist=ABCDEFGHIJKLMNOPQRSTUVWXYZ --oem 3 --psm 7").strip().replace("1","I").replace("5","s").replace("0","o").replace("PL","il").replace("|","i").replace(" ", "").replace("\n", "").upper()

    print(f"1 {prompt}")
elif "click" in other:
    print(f"2 {x} {y} {w} {h}")
else :
    print("0")

