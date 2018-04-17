import           Control.Monad

-- coins.hs --- Coins

-- author: Seong Yong-ju <sei40kr@gmail.com>

solve :: Int -> Int -> Int -> Int -> Int
solve a b c x = length
      [ ()
      | u <- [0 .. a]
      , v <- [0 .. b]
      , w <- [0 .. c]
      , 500 * u + 100 * v + 50 * w == x
      ]

main :: IO ()
main = do
  [a, b, c, x] <- replicateM 4 $ (read :: String -> Int) <$> getLine
  print $ solve a b c x
