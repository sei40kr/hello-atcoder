import           Data.List

-- task-scheduling-problem.hs --- Task Scheduling Problem

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve ::[Int] ->Int
solve [a1, a2, a3] = (sum . take 2 . sort) ds
  where
    ds = [abs $ a1 - a2, abs $ a2 - a3, abs $ a3 - a1]

main :: IO ()
main = do
  as <- map (read :: String -> Int) . take 3 . words <$> getLine
  print $ solve as
