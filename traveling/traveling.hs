import           Control.Monad

-- traveling.hs ---

-- author: Seong Yong-ju <sei40kr@gmail.com>

toTuple3 :: [Int] -> (Int, Int, Int)
toTuple3 (x:y:z:_) = (x, y, z)

solve :: [(Int, Int, Int)] -> Bool
solve [] = True
solve [_] = True
solve ((t, x, y):(t', x', y'):txys) =
  dist <= dt && mod (dt - dist) 2 == 0 && solve ((t', x', y') : txys)
  where
    dt = t' - t
    dist = abs (x' - x) + abs (y' - y)

main :: IO ()
main = do
  n <- (read :: String -> Int) <$> getLine
  txys <- replicateM n $ toTuple3 . map (read :: String -> Int) . take 3 . words <$> getLine
  putStrLn $ if solve ((0,0,0):txys) then "Yes" else "No"
