import           Data.List

-- many-medians.hs --- Many Medians

-- author: Seong Yong-ju <sei40kr@gmail.com>


-- TODO Make this work on AtCoder environment (GHC 7.10.3)

solve :: [Int] -> [Int]
solve xs = map ((!!) [vr, vl] . flip quot vr) xs
  where
    mr = quot (length xs) 2
    sorted = sort xs
    vl = sorted !! mr
    vr = sorted !! mr + 1

main :: IO ()
main = do
  n <- (read :: String -> Int) <$> getLine
  xs <- take n . map (read :: String -> Int) . words <$> getLine
  putStrLn $ unwords $ map show $ solve xs
