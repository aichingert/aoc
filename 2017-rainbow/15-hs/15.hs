module Main where

import Data.Bits

genA :: Int -> [Integer]
genA n = let l = take (n + 1) (iterate (\x -> mod (x * 16807) 2147483647) 65) 
                    in map (.&. 0xFFFF) l

genB :: Int -> [Integer]
genB n = let l = take (n + 1) (iterate (\x -> mod (x * 48271) 2147483647) 8921) 
                    in map (.&. 0xFFFF) l

genA' :: Int -> [Integer]
genA' n = let l = take (n + 1) (iterate (\x -> mod (x * 16807) 2147483647) 65) 
                    in map (.&. 0xFFFF) l

partOne :: Int -> Int
partOne n = length (filter (== 0) (zipWith (xor) (genA n) (genB n)))

isValidA :: Integer -> Bool
isValidA x = (mod x 4) == 0

isValidB :: Integer -> Bool
isValidB x = (mod x 8) == 0

fiveK :: Int -> Int
fiveK n = length (filter (isValidB) (genB n))

main :: IO ()
main = do
    putStrLn ("Part one: " ++ (show (partOne 40000000)))
    putStrLn ("Part two: " ++ (show (fiveK 41000000)))
