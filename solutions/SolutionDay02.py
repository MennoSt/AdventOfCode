from utils.AocUtils import *
from utils.FileReader import *
from enum import Enum

class Hand(Enum):
    ROCK = 0
    PAPER = 1
    SCIZZOR = 2

def convertStrToHand(str):
    if str == "A" or str == "X":
        return Hand.ROCK
    if str == "B" or str == "Y":
        return Hand.PAPER
    if str == "C" or str == "Z":
        return Hand.SCIZZOR
    
def roundScore(elf, me):
    elf = convertStrToHand(elf)
    me = convertStrToHand(me)
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

def calculateScore(strategyFile):
    totalScore = 0
    for game in strategyFile:
        totalScore += roundScore(game[0], game[1])
    
    return totalScore
        
def solutionDay02():
    
    fileReader = FileReader()
    strategy = fileReader.readToStringMap("input/inputday2", True)
    answer1 = calculateScore(strategy)
    answer2 = 0
    printAnswer(2, answer1, answer2)