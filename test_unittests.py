import unittest

from solutions.SolutionDay01 import *


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

if __name__ == '__main__':
    unittest.main()
        
