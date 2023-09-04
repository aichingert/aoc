module AOC where

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


