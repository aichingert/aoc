module Main where

import Data.List
import Data.Char

get_sides :: [Char] -> Int -> [Int]
get_sides [] acc = acc : []
get_sides (x:xs) acc    | x == 'x' = acc : get_sides xs 0
                        | otherwise = get_sides xs ((ord x - 48) + acc * 10)

part_one :: [[Char]] -> Int
part_one [] = 0
part_one (x:xs) = case (sort (get_sides x 0)) of
    [l, w, h] -> 2 * l * w + 2 * w * h + 2 * h * l + l * w + part_one xs
    _ -> 0

part_two :: [[Char]] -> Int
part_two [] = 0
part_two (x:xs) = case (sort (get_sides x 0)) of
    [l, w, h] -> 2*l + 2*w + l*w*h + part_two xs
    _ -> 0

main :: IO ()
main = do
    inp <- readFile "../../../input/2015/02"
    let l = lines inp
    putStrLn ("Part one: " ++ (show (part_one l)))
    putStrLn ("Part two: " ++ (show (part_two l)))
