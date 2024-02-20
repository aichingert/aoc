module Main where

import Data.Bits

dropFirst :: [Integer] -> [Integer]
dropFirst [] = []
dropFirst (x:xs) = xs

gen :: Int -> Integer -> Integer -> [Integer]
gen n fac start = let l = take (n + 1) (iterate (\x -> mod (x * fac) 2147483647) start) 
                    in map (.&. 0xFFFF) (dropFirst l)

partOne :: Int -> Int
partOne n = length (filter (== 0) (zipWith (xor) (gen n 16807 65) (gen n 48271 8921)))

main :: IO ()
main = do
    putStrLn ("Part one: " ++ (show (partOne 40000000)))
