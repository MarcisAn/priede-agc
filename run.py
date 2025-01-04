import subprocess
import time
import os
import shutil
# subprocess.run(["../VirtualAGC/bin/yaYUL.exe", "../dist/Validation.agc"])

# time.sleep(2)
subprocess.run(["VirtualAGC/bin/yaYUL.exe", "dist/Validation.agc"])
shutil.move("E:/Dev/priede-agc/dist/Validation.agc.bin", "C:/Users/marcis/Downloads/VirtualAGC-Windows-2017-03-29.tar/VirtualAGC-Windows-2017-03-29/VirtualAGC/Resources/source/Validation/Validation.bin")
shutil.move("E:/Dev/priede-agc/dist/Validation.agc.symtab", "C:/Users/marcis/Downloads/VirtualAGC-Windows-2017-03-29.tar/VirtualAGC-Windows-2017-03-29/VirtualAGC/Resources/source/Validation/Validation.bin.symtab")
shutil.copy("E:/Dev/priede-agc/dist/Validation.agc", "C:/Users/marcis/Downloads/VirtualAGC-Windows-2017-03-29.tar/VirtualAGC-Windows-2017-03-29/VirtualAGC/Resources/source/Validation/Validation.agc")


# os.system("cmd /c move \"E:/Dev/priede-agc/dist/Validation.agc.bin\" \"C:/Users/marcis/Downloads/VirtualAGC-Windows-2017-03-29.tar/VirtualAGC-Windows-2017-03-29/VirtualAGC/Resources/source/Validation/Validation.bin\"")
#subprocess.run(["MOVE", "dist/Validation.agc.bin", "C:\Users\marcis\Downloads\VirtualAGC-Windows-2017-03-29.tar\VirtualAGC-Windows-2017-03-29\VirtualAGC\Resources\source\Validation/Validation.bin"])
