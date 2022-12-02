from utils.AocUtils import *
from utils.FileReader import *
from enum import Enum

class Hand(Enum):
    ROCK = 0
    PAPER = 1
    SCIZZOR = 2

class Result (Enum):
    WIN = 0
    LOSE = 1
    DRAW = 2

def convertStrToResult(str):
    if str == "X":
        return Result.LOSE
    if str == "Y":
        return Result.DRAW
    if str == "Z":
        return Result.WIN

def convertStrToHand(str):
    if str == "A" or str == "X":
        return Hand.ROCK
    if str == "B" or str == "Y":
        return Hand.PAPER
    if str == "C" or str == "Z":
        return Hand.SCIZZOR

def calculateTotalScorePart1(strategyFile):
    totalScore = 0
    for game in strategyFile:
        totalScore += roundScore(convertStrToHand(game[0]), convertStrToHand(game[1]))
    
    return totalScore

def roundScore(elf, me):
    score = 0
    if me == Hand.ROCK and elf == Hand.SCIZZOR:
        score += 6
    elif me == Hand.PAPER and elf == Hand.ROCK:
        score += 6
    elif me == Hand.SCIZZOR and elf == Hand.PAPER:
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
    
    if expectedResult == Result.LOSE:
        if handElf == Hand.PAPER:
            return Hand.ROCK
        if handElf == Hand.ROCK:
            return Hand.SCIZZOR
        if handElf == Hand.SCIZZOR:
            return Hand.PAPER
    
    if expectedResult == Result.WIN:
        if handElf == Hand.ROCK:
            return Hand.PAPER
        if handElf == Hand.PAPER:
            return Hand.SCIZZOR
        if handElf == Hand.SCIZZOR:
            return Hand.ROCK
    
    if expectedResult == Result.DRAW:
        return handElf
    


def calculateTotalScorePart2(strategyFile):
    totalScore = 0
    for game in strategyFile:
        handElf = convertStrToHand(game[0])
        expectedResult = convertStrToResult(game[1])
        myHand = determineHandToPlay(handElf, expectedResult)
        totalScore += roundScore(convertStrToHand(game[0]), myHand)
    
    return totalScore

def solutionDay02():
    
    fileReader = FileReader()
    strategy = fileReader.readToStringMap("input/inputday2", True)
    answer1 = calculateTotalScorePart1(strategy)
    answer2 = calculateTotalScorePart2(strategy)
    printAnswer(2, answer1, answer2)