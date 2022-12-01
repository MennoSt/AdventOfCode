import unittest


from solutions.SolutionDay01 import *


class Test_SumbarineCalculator(unittest.TestCase):

    def setUp(self):
        self.calorieCalculator = CalorieCalculator()
        self.fileReader = FileReader()
    
    def test_CalculateDepthIncreases_Part1_Example1(self):
        calories = self.fileReader.readLinesToStringArray("input_ut/inputday1_ut")
        maxCalories = self.calorieCalculator.calculateMaxCalories(calories)
        self.assertEqual(maxCalories, 9)

        
