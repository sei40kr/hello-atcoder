import Control.Monad;
import Control.Applicative;

-- bulletin-board.hs --- Bulletin Board

-- author: Seong Yong-ju <sei40kr@gmail.com>

main :: IO ()
main = do
  [n, h, w] <- replicateM 3 $ (read :: String -> Int) <$> getLine
  print $ (n - h + 1) * (n - w + 1)
