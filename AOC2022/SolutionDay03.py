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
sum = calculateSumCommonCharacters(inputArray)
print(sum)

inputArray = fileReader.readLinesToStringArray("input_ut/inputday3_ut")
sum = calculateSumCommonCharacters(inputArray)
assert sum == 157
# assert sumNBiggestEaters(inputArray, 3) == 157

# %%
