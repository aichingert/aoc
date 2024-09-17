module Main where

import qualified Data.Set as S
import Data.Bool

update :: Char -> [Int] -> [Int]
update c [x, y] | c == '<' = [(x - 1), y]
                | c == '>' = [(x + 1), y]
                | c == 'v' = [x, (y - 1)]
                | c == '^' = [x, (y + 1)]
                | otherwise = [x, y]

part_one :: [Char] -> [Int] -> [[Int]]
part_one [] _ = [[0, 0]]
part_one (c:xs) arr = case (update c arr) of nxt -> nxt : part_one xs nxt

part_two :: [Char] -> Bool -> [Int] -> [Int] -> [[Int]]
part_two [] _ _ _ = [[0, 0]]
part_two (c:xs) is_santa santa robo = case is_santa of 
    True ->  case (update c santa) of nxt -> nxt : part_two xs False nxt robo
    False -> case (update c robo)  of nxt -> nxt : part_two xs True santa nxt

main :: IO ()
main = do
    inp <- readFile "../../../input/2015/03"
    putStrLn $ show $ S.size $ S.fromList $ part_one inp [0, 0]
    putStrLn $ show $ S.size $ S.fromList $ part_two inp True [0, 0] [0, 0]

