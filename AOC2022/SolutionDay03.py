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

def determineCommonType(input):
    array = splitCompartments(input)
    commonCharacters = []
    for compartment in array:
        commonCharacters.append(''.join(set(compartment[0]).intersection(compartment[1])))

    return commonCharacters

def calculateSumCommonCharacters(input):
    commonCharacters = determineCommonType(input)
    return calculateScore(commonCharacters)

def splitCompartmentsPart2(input):
    array = []
    for i in range(0,len(input)):
        if i % 3 == 0:
            array.append([input[i],input[i+1], input[i+2]])
    return array

def determineCommonTypePart2(input):
    array = splitCompartmentsPart2(input)
    commonCharacters = []
    for compartment in array:
        commonChar = ''.join(set(compartment[0]).intersection(compartment[1]))
        commonCharacters.append(''.join(set(commonChar).intersection(compartment[2])))

    return commonCharacters

def calculateSumCommonCharactersPart2(input):
    commonCharacters = determineCommonTypePart2(input)
    return calculateScore(commonCharacters)


def calculateScore(commonCharacters):
    score = 0
    for character in commonCharacters:
        asciiNumber = ord(character)
        if asciiNumber < 123 and asciiNumber > 96:
            score+= (asciiNumber - 96)
        else:
            score+= (asciiNumber - 64 + 26) # starts at 27

    return score


# %%
# Solution
inputArray = fileReader.readLinesToStringArray("input/inputday3")
answer1 = calculateSumCommonCharacters(inputArray)
answer2 = calculateSumCommonCharactersPart2(inputArray)
printAnswer(3, answer1, answer2)
assert answer1 == 7917
assert answer2 == 2585

# %%
# Example Tests
inputArray = fileReader.readLinesToStringArray("input_ut/inputday3_ut")
sum1 = calculateSumCommonCharactersPart2(inputArray)
assert sum1 == 70
sum2 = calculateSumCommonCharacters(inputArray)
assert sum2 == 157

# %%
