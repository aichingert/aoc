module Main where

import Data.List

tail' :: [a] -> [a]
tail' (_:xs) = xs
tail' [] = []

splitOn :: Eq a => a -> [a] -> [[a]]
splitOn _ [] = []
splitOn d s = x:splitOn d (tail' s')
  where (x, s') = span (/= d) s

lineFromString :: String -> [Int]
lineFromString s = sort (read l :  read w : read h : [])
  where [l, w, h] = splitOn 'x' s

getWrappingPaper :: [Int] -> Int
getWrappingPaper (l:w:h:[]) = 2 * l * w + 2 * w * h + 2 * h * l + l * w

getRibbon :: [Int] -> Int
getRibbon (l:w:h:[]) = 2*l + 2*w + l*w*h

partOne :: [String] -> Int
partOne inp = sum $ map getWrappingPaper $ map lineFromString inp

partTwo :: [String] -> Int
partTwo inp = sum $ map getRibbon $ map lineFromString inp

main = do
  input <- readFile "../input/02"
  let dimensions = lines input
  putStrLn $ show $ partOne dimensions
  putStrLn $ show $ partTwo dimensions

