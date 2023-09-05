module Main where 

import Data.List

data Point = Point { x :: Int
                   , y :: Int
                   } deriving (Show, Eq, Ord)

move :: Char -> Point -> Point
move c (Point {x = px, y = py}) = case c of
             '>' -> Point { x = px + 1, y = py }
             '<' -> Point { x = px - 1, y = py }
             'v' -> Point { x = px, y = py + 1 }
             '^' -> Point { x = px, y = py - 1 }

followPath :: Point -> [Char] -> [Point]
followPath p [] = []
followPath p (x:xs) = let np = move x p in np : followPath np xs

countVisitedHouses :: [Point] -> Int
countVisitedHouses = length . map head . group . sort

partOne :: [Char] -> Int
partOne [] = 0
partOne xs = countVisitedHouses (let p = Point { x = 0, y = 0 } in p : followPath p xs)

main :: IO ()
main = do
  input <- readFile "../input/03"
  putStrLn $ show $ partOne $ init input
