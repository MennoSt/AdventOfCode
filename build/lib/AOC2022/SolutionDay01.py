# %%
from utils.AocUtils import *
from utils.FileReader import *

def sumNBiggestEaters(inputCals, n):
    tmpVal = 0
    calorieArray = []
    
    for cal in inputCals:
        tmpVal += cal
        if cal == 0:
            calorieArray.append(tmpVal)
            tmpVal = 0

    calorieArray.append(tmpVal)
    calorieArray.sort(reverse=True)
    
    return sum(calorieArray[:n])

# %%
# Solution
fileReader = FileReader()
inputArray = fileReader.readLinesToIntArray("input/inputday1")
answer1 = sumNBiggestEaters(inputArray, 1)
answer2 = sumNBiggestEaters(inputArray, 3)
printAnswer(1, answer1, answer2)
assert answer1 == 70698
assert answer2 == 206643

# %%
# Example
inputArray = fileReader.readLinesToIntArray("inputut/inputday1_ut")
assert sumNBiggestEaters(inputArray, 1) == 24000
assert sumNBiggestEaters(inputArray, 3) == 45000

# %%
