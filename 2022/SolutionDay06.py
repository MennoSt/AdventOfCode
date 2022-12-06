# %%
from utils.AocUtils import *
from utils.FileReader import *

def marker(inputStr):
    
    for i in range(0, len(inputStr)):
        tmpStr = inputStr[i:i+4]
        if (len(set(tmpStr)) == len(tmpStr)):
            print("marker reached "+str(i+4))
            return (i+4)
    
    return 0


def marker2(inputStr):
    
    for i in range(0, len(inputStr)):
        tmpStr = inputStr[i:i+14]
        if (len(set(tmpStr)) == len(tmpStr)):
            print("marker reached "+str(i+14))
            return (i+14)
    
    return 0

# %%
# Example Tests
assert marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 7

# %%


# %%
# Solution
inputStr = open("input/inputday6").read()
assert marker(inputStr) == 1262
assert marker2(inputStr) == 3444


marker2(inputStr)