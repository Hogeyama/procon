{-# LANGUAGE Strict #-}
{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE MultiWayIf #-}
{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE ScopedTypeVariables #-}

{-import           Data.Set (Set)-}
{-import qualified Data.Set as S-}
{-import           Data.Map (Map)-}
{-import qualified Data.Map as M-}
import           Data.Array.IArray
import           Data.Array.MArray
import           Data.Array.ST
import           Control.Monad (forM_, replicateM)
import           Control.Monad.ST
import           Control.Lens.Lens ((<&>))

readInt :: String -> Int
readInt = read

getInts :: IO [Int]
getInts = map readInt . words <$> getLine

main :: IO ()
main = do
  [n,limit] <- getInts
  (ws,vs) <- fmap unzip . replicateM n $ do
    [w,v] <- getInts
    return (w,v)
  {-print $ knapsack n limit ws vs-}
  print $ knapsack2 n limit ws vs

knapsack :: Int -> Int -> [Int] -> [Int] -> Int
knapsack n limit ws vs = runST $ do
  dp <- thaw' $ listArray ((0,0),(n,limit)) (repeat (-1))
  f dp 0 limit
  where
    f :: forall s. (STArray s (Int,Int) Int) -> Int -> Int -> ST s Int
    f dp i w = do
      v <- readArray dp (i,w)
      if| v >= 0     -> return v
        | i == n     -> writeRet 0
        | w <  ws!!i -> writeRet =<< f dp (i+1) w
        | otherwise  -> do
            x <- f dp (i+1) w
            y <- f dp (i+1) (w-ws!!i) <&> (+(vs!!i))
            writeRet $ max x y
      where
        writeRet :: Int -> ST s Int
        writeRet v = writeArray dp (i,w) v >> return v

knapsack2 :: Int -> Int -> [Int] -> [Int] -> Int
knapsack2 n limit ws vs = runST $ do
  dp <- thaw' $ listArray ((0,0),(n,limit)) (repeat 0)
  forM_ (reverse [0..n-1]) $ \i -> do
    forM_ [0..limit] $ \w -> do
      let read  = readArray dp
          write = writeArray dp
      if| w < ws!!i ->  write (i,w) =<< read (i+1,w)
        | otherwise -> do
            x <- read (i+1,w)
            y <- read (i+1,w-ws!!i) <&> (+(vs!!i))
            write (i,w) $ max x y
  readArray dp (0,limit)

thaw' :: Ix i => Array i e -> ST s (STArray s i e)
thaw' = thaw

