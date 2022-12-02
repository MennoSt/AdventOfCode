from utils.AocUtils import *
from utils.AocEnums import *
from utils.FileReader import *

def roundScore(elf, me):
    score = 0
    if me == Hand.ROCK and elf == Hand.SCIZZOR or\
        me == Hand.PAPER and elf == Hand.ROCK or\
        me == Hand.SCIZZOR and elf == Hand.PAPER:
        score += 6
    elif me == elf:
        score += 3
    
    if me == Hand.ROCK:
        score += 1
    elif me == Hand.PAPER:
        score += 2
    elif me == Hand.SCIZZOR:
        score += 3

    return score

def determineHandToPlay(handElf, expectedResult):
    if expectedResult == Result.DRAW:
        return handElf
    
    if expectedResult == Result.LOSE:
        if handElf == Hand.PAPER:
            return Hand.ROCK
        elif handElf == Hand.ROCK:
            return Hand.SCIZZOR
        elif handElf == Hand.SCIZZOR:
            return Hand.PAPER
    
    if expectedResult == Result.WIN:
        if handElf == Hand.ROCK:
            return Hand.PAPER
        elif handElf == Hand.PAPER:
            return Hand.SCIZZOR
        elif handElf == Hand.SCIZZOR:
            return Hand.ROCK

def calculateTotalScore(strategyFile, withCorrectStrategy=True):
    totalScore = 0
    for game in strategyFile:
        handElf = convertStrToHand(game[0])
        if withCorrectStrategy:
            expectedResult = convertStrToResult(game[1])
            myHand = determineHandToPlay(handElf, expectedResult)
            totalScore += roundScore(convertStrToHand(game[0]), myHand)
        else:
            myHand = convertStrToHand(game[1])
            totalScore += roundScore(handElf, myHand)
    return totalScore

def solutionDay02():
    
    fileReader = FileReader()
    strategy = fileReader.readToStringMap("input/inputday2", True)
    answer1 = calculateTotalScore(strategy, False)
    answer2 = calculateTotalScore(strategy)
    printAnswer(2, answer1, answer2)