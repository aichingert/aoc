module Main where 

data Point = Point { x :: Int
                   , y :: Int
                   } deriving (Show)

move :: Char -> Point -> Point
move c (Point {x = px, y = py}) = case c of
             '>' -> Point { x = px + 1, y = py }
             '<' -> Point { x = px - 1, y = py }
             'v' -> Point { x = px, y = py + 1 }
             '^' -> Point { x = px, y = py - 1 }

partOne :: [Char] -> Int
partOne [] = 0
partOne xs = 10

main = do
  input <- readFile "../input/03"
  putStrLn $ show $ move '>' Point { x = 10, y = 20 }
  putStrLn $ show $ partOne input
