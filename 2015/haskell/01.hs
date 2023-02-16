-- Advent of Code 2015, day 1
-- (c) aichingert

module Main where

import Data.Char

part1 :: [Char] -> Int
part1 [] = 0
part1 (x:xs) = case x of
  '(' -> part1 xs + 1
  ')' -> part1 xs - 1
  _ -> part1 xs

main :: IO ()
main = do
  input <- readFile "../input/01"
  print $ part1 input


