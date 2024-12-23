module Main where

parse :: [String] -> [[Int]]
parse [x]
parse (x:xs) = 

main :: IO ()
main = do 
    inp <- readFile "../../../input/2024/01"
    putStrLn ("Part one: " ++ (show (part_one inp 0)))
    putStrLn ("Part two: " ++ (show (part_two inp 0 0)))
