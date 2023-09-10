module Main where

import Data.List

isVowel :: Char -> Bool
isVowel c = elem c "aeiou"

countVowels :: String -> Int
countVowels = length . filter isVowel

containsInvalidCombination :: String -> Bool
containsInvalidCombination [x] = False 
containsInvalidCombination (f:xs) = let s = head xs 
    in case (f, s) of
            ('a', 'b') -> True
            ('c', 'd') -> True
            ('p', 'q') -> True
            ('x', 'y') -> True
            otherwise  -> containsInvalidCombination xs

containsDouble :: String -> Bool
containsDouble [x] = False
containsDouble (x:xs) = x == head xs || containsDouble xs

isNiceString :: String -> Bool
isNiceString s = countVowels s > 2 && not (containsInvalidCombination s) && containsDouble s

containsRepeatingLetter :: String -> Bool
containsRepeatingLetter [] = False
containsRepeatingLetter [_] = False
containsRepeatingLetter [_, _] = False
containsRepeatingLetter (x:xs) = x == head (tail xs) || containsRepeatingLetter xs

fy :: [Char] -> String -> Bool
fy xs [] = False
fy xs [x] = False
fy (f:s) (x:xs) = f == x && head s == head xs || fy [f, head s] xs

fx :: String -> Bool
fx [x] = False
fx (f:xs) = fy [f, head xs] (tail xs) || fx xs

isNice :: String -> Bool 
isNice s = fx s && containsRepeatingLetter s

partOne :: [String] -> Int
partOne = length . filter isNiceString

partTwo :: [String] -> Int
partTwo = length . filter isNice

main :: IO ()
main = do
  input <- readFile "input/05"
  let inp = lines input 
  putStrLn $ show $ partOne inp
  putStrLn $ show $ partTwo inp
