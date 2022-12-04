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
        pair = [{"min":int(input1[0]),"max":int(input1[1])},{"min":int(input2[0]),"max":int(input2[1])}]
        pairArray.append(pair)   
    return pairArray

def pairContainsOther(min1, max1, min2, max2):
    if min1 <= min2 and max1 >= max2:
        return True
    elif min2 <= min1 and max2 >= max1:
        return True
    else:
        return False

def pairInRange(min1, max1, min2, max2):
    if min1 >= min2 and min1 <= max2:
        return True
    elif max1 >= min2 and max1 <= max2:
        return True
    elif min2 >= min1 and min2 <= max1:
        return True
    elif max2 >= min1 and max2 <= max1:
        return True
    else:
        return False
        
def countOverlappingPairs(pairArray):
    count = 0
    for pair in pairArray:
        if (pairContainsOther(pair[0]["min"], pair[0]["max"], pair[1]["min"], pair[1]["max"])):
            count += 1
    
    return count

def countRangeOverlaps(pairArray):
    count = 0
    for pair in pairArray:
        if (pairInRange(pair[0]["min"], pair[0]["max"], pair[1]["min"], pair[1]["max"])):
            count += 1
    return count

# %%
# Solution
inputArray = parseValues("input/inputday4")
answer1 = countOverlappingPairs(inputArray)
answer2 = countRangeOverlaps(inputArray) 
printAnswer(3, answer1, answer2)
assert answer1 == 459
assert answer2 == 779

# %%
# Example Tests
pairArray = parseValues("input_ut/inputday4_ut")
assert countOverlappingPairs(pairArray) == 2
assert countRangeOverlaps(pairArray) == 4
# %%
