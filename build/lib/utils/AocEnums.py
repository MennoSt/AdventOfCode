from enum import Enum

class Direction(Enum):
    CURRENT = 0
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4

class char(str, Enum):
    bracketOpen = "{"
    roundOpen = "("
    arrowOpen = "<"
    squareOpen = "["
    arrowClosed = ">"
    roundClosed = ")"
    bracketClosed = "}"
    squareClosed = "]"

class Operator(Enum):
    MINUSX = 0
    PLUSX = 1
    MINUSY = 2
    PLUSY = 3
    MINUSZ = 4
    PLUSZ = 5

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