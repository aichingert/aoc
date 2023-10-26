module Aoc where

import Data.Char

tail' :: [a] -> [a]
tail' (_:xs) = xs
tail' [] = []

splitOn :: Eq a => a -> [a] -> [[a]]
splitOn _ [] = []
splitOn d s = x:splitOn d (tail' s')
  where (x, s') = span (/= d) s

