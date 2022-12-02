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
    
    def test_CalculateScoreInputUTpart1(self):
            strategyGuide = self.fileReader.readToStringMap("input_ut/inputday2_ut", True)
            score = calculateTotalScorePart1(strategyGuide)
            self.assertEqual(score, 15)

    def test_CalculateScoreInputUTpart2(self):
            strategyGuide = self.fileReader.readToStringMap("input_ut/inputday2_ut", True)
            score = calculateTotalScorePart2(strategyGuide)
            self.assertEqual(score, 12)


if __name__ == '__main__':
    unittest.main()
        
