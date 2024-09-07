module Main where

part_one :: [Char] -> Int -> Int
part_one [] loc = loc
part_one (x:xs) loc | x == '(' = part_one xs (loc + 1)
                    | x == ')' = part_one xs (loc - 1)
                    | x /= '(' && x /= ')' = part_one xs loc

part_two :: [Char] -> Int -> Int -> Int
part_two [] idx loc = idx
part_two (x:xs) idx loc | loc < 0 = idx
                        | x == '(' = part_two xs (idx + 1) (loc + 1)
                        | x == ')' = part_two xs (idx + 1) (loc - 1)
                        | x /= '(' && x /= ')' = part_two xs idx loc

main :: IO ()
main = do 
    inp <- readFile "../../../input/2015/01"
    putStrLn ("Part one: " ++ (show (part_one inp 0)))
    putStrLn ("Part two: " ++ (show (part_two inp 0 0)))
