# %%
from utils.AocUtils import *
from utils.FileReader import *
fileReader = FileReader()

def splitCompartments(input):
    array = []
    for line in input:
        part1 = line[:len(line)//2]
        part2 = line[len(line)//2:]
        array.append([part1,part2])
    return array

def divideInGroupsOfThree(input):
    array = []
    for i in range(0,len(input)):
        if i % 3 == 0:
            array.append([input[i], input[i+1], input[i+2]])
    return array

def calculateScore(commonCharacters):
    score = 0
    for character in commonCharacters:
        asciiNumber = ord(character)
        if asciiNumber < 123 and asciiNumber > 96:
            score+= (asciiNumber - 96)
        else:
            score+= (asciiNumber - 64 + 26) # starts at 27

    return score

def getCommonCharacters(stringOne, stringTwo):
    return ''.join(set(stringOne).intersection(stringTwo))

def determineCommonCharacters(array):
    commonCharacters = []
    for compartment in array:
        commonChar = compartment[0]
        for i in range(1, len(compartment)):
            commonChar = getCommonCharacters(commonChar, compartment[i])
        commonCharacters.append(commonChar)

    return commonCharacters

def calculateSumCommonCharacters(input, split = False):
    if split == True:
        array = splitCompartments(input)
    else:
        array = divideInGroupsOfThree(input)
    
    commonCharacters = determineCommonCharacters(array)
    return calculateScore(commonCharacters)

# %%
# Example Tests
inputArray = fileReader.readLinesToStringArray("input_ut/inputday3_ut")
assert calculateSumCommonCharacters(inputArray, True) == 157
assert calculateSumCommonCharacters(inputArray) == 70

# %%
# Solution
inputArray = fileReader.readLinesToStringArray("input/inputday3")
answer1 = calculateSumCommonCharacters(inputArray, True)
answer2 = calculateSumCommonCharacters(inputArray)
printAnswer(3, answer1, answer2)
assert answer1 == 7917
assert answer2 == 2585
