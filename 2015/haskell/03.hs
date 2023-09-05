module Main where 

import Data.List

data Point = Point { x :: Int
                   , y :: Int
                   } deriving (Eq, Ord)

starting :: Point
starting = Point { x = 0, y = 0 }

move :: Char -> Point -> Point
move c (Point {x = px, y = py}) = case c of
    '>' -> Point { x = px + 1, y = py }
    '<' -> Point { x = px - 1, y = py }
    'v' -> Point { x = px, y = py + 1 }
    '^' -> Point { x = px, y = py - 1 }

followPath :: Point -> [Char] -> [Point]
followPath p [] = []
followPath p (x:xs) = let np = move x p in np : followPath np xs

followPathWithTwo :: (Point, Point) -> Int -> [Char] -> [Point]
followPathWithTwo (_, _) _ [] = [] 
followPathWithTwo (roboSanta, santa) isSantasTurn (x:xs) = case isSantasTurn of 
    0 -> let np = move x santa in np : followPathWithTwo (roboSanta, np) 1 xs
    1 -> let np = move x roboSanta in np : followPathWithTwo (np, santa) 0 xs

countVisitedHouses :: [Point] -> Int
countVisitedHouses = length . map head . group . sort

partOne :: [Char] -> Int
partOne [] = 0
partOne xs = countVisitedHouses (let p = starting in p : followPath p xs)

partTwo :: [Char] -> Int
partTwo xs = countVisitedHouses (let p = starting in 
    p : followPathWithTwo (starting, starting) 0 xs)

main :: IO ()
main = do
  input <- readFile "../input/03"
  putStrLn $ show $ partOne $ init input
  putStrLn $ show $ partTwo $ init input
