# %%
from utils.AocUtils import *
from utils.FileReader import *

def marker(inputStr, size):
    for i in range(0, len(inputStr)):
        tmpStr = inputStr[i:i+size]
        if (len(set(tmpStr)) == len(tmpStr)):
            return (i+size)

# %%
# Example Tests
assert marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4) == 7

# %%
# Solution
inputStr = open("input/inputday6").read()
assert marker(inputStr, 4) == 1262
assert marker(inputStr, 14) == 3444
