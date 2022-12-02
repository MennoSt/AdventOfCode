import unittest

from solutions.SolutionDay01 import *
from solutions.SolutionDay02 import *


class Test_Day01(unittest.TestCase):

    def setUp(self):
        self.fileReader = FileReader()
    
    def test_CalculateMaxCaloriesInputUT(self):
        calories = self.fileReader.readLinesToIntArray("input_ut/inputday1_ut")
        
        maxCalories = sumNBiggestEaters(calories, 1)
        self.assertEqual(maxCalories, 24000)

    def test_SumBiggestThreeEatersInputUT(self):
        calories = self.fileReader.readLinesToIntArray("input_ut/inputday1_ut")
        
        maxCalories = sumNBiggestEaters(calories, 3)
        self.assertEqual(maxCalories, 45000)

    def test_CalculateMaxCaloriesInput(self):
        calories = self.fileReader.readLinesToIntArray("input/inputday1")
        
        maxCalories = sumNBiggestEaters(calories, 1)
        self.assertEqual(maxCalories, 70698)
        
    def test_SumBiggestThreeEatersInput(self):
        calories = self.fileReader.readLinesToIntArray("input/inputday1")
        
        maxCalories = sumNBiggestEaters(calories, 3)
        self.assertEqual(maxCalories, 206643)

class Test_Day02(unittest.TestCase):

    def setUp(self):
        self.fileReader = FileReader()
    
    def test_CalculateScoreIncorrectStrategyUT(self):
            strategyGuide = self.fileReader.readToStringMap("input_ut/inputday2_ut", True)
            score = calculateTotalScore(strategyGuide, False)
            self.assertEqual(score, 15)

    def test_CalculateScoreCorrectStrategyUT(self):
            strategyGuide = self.fileReader.readToStringMap("input_ut/inputday2_ut", True)
            score = calculateTotalScore(strategyGuide)
            self.assertEqual(score, 12)

    def test_CalculateScoreIncorrectStrategy(self):
            strategyGuide = self.fileReader.readToStringMap("input/inputday2", True)
            score = calculateTotalScore(strategyGuide, False)
            self.assertEqual(score, 11603)

    def test_CalculateScoreCorrectStrategy(self):
            strategyGuide = self.fileReader.readToStringMap("input/inputday2", True)
            score = calculateTotalScore(strategyGuide)
            self.assertEqual(score, 12725)


if __name__ == '__main__':
    unittest.main()
        
