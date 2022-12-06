# %%
from utils.AocUtils import *
from utils.FileReader import *
fileReader = FileReader()

def parseValues(inputFile):
    inputArray = fileReader.readLinesToStringArray(inputFile)
    pairArray = []
    for input in inputArray:
        input = input.split(",")
        input1 = input[0].split("-")
        input2 = input[1].split("-")
        pair = [{"min":int(input1[0]),"max":int(input1[1])}\
            ,{"min":int(input2[0]),"max":int(input2[1])}]
        pairArray.append(pair)   
    return pairArray

def pairContainsOther(min1, max1, min2, max2):
    if min1 <= min2 and max1 >= max2 or\
        min2 <= min1 and max2 >= max1:
        return True
    else:
        return False

def pairInRange(min1, max1, min2, max2):
    if min1 >= min2 and min1 <= max2 or\
        max1 >= min2 and max1 <= max2 or\
        min2 >= min1 and min2 <= max1 or\
        max2 >= min1 and max2 <= max1:
        return True
    else:
        return False

def countPairs(pairArray):
    countContain = 0
    countRange = 0 
    for pair in pairArray:
        if (pairContainsOther(pair[0]["min"], pair[0]["max"],\
             pair[1]["min"], pair[1]["max"])):
            countContain += 1
        if (pairInRange(pair[0]["min"], pair[0]["max"],\
             pair[1]["min"], pair[1]["max"])):
            countRange += 1
    return [countContain, countRange]

# %%
# Example Tests
pairArray = parseValues("input_ut/inputday4_ut")
assert countPairs(pairArray) == [2,4]
# %%

# %%
# Solution
inputArray = parseValues("input/inputday4")
count = countPairs(inputArray)
printAnswer(3, count[0], count[1])
assert count == [459, 779]