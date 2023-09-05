-- Advent of Code 2015, day 1
-- (c) aichingert

module Main where

import Data.Char

part1 :: [Char] -> Int -> Int
part1 [] loc = loc
part1 (x:xs) loc | x == '(' = part1 xs (loc + 1)
                 | x == ')' = part1 xs (loc - 1)

part2 :: [Char] -> Int -> Int -> Int
part2 [] loc cur | loc < 0 = cur 
                 | otherwise = -1
part2 (x:xs) loc cur | loc < 0 = cur
                     | x == '(' = part2 xs (loc + 1) (cur + 1)
                     | x == ')' = part2 xs (loc - 1) (cur + 1)

main :: IO ()
main = do
  input <- readFile "input/01"
  putStrLn ("Part 1: " ++ (show (part1 input 0)))
  putStrLn ("Part 2: " ++ (show (part2 input 0 0)))
