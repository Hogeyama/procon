{-# LANGUAGE BangPatterns #-}

--import           Data.Set (Set)
--import qualified Data.Set as S
--import           Data.Map (Map)
--import qualified Data.Map as M
import           Data.Vector (Vector,(!))
import qualified Data.Vector as V
import           Data.Vector.Mutable (write)
import qualified Data.Vector.Mutable as MV

readInt :: String -> Int
readInt = read

getInt :: IO Int
getInt = readInt <$> getLine

getInts :: IO [Int]
getInts = map readInt . words <$> getLine

main :: IO ()
main = do
  n <- getInt
  as <- getInts
  print (n,as)

lis :: Int -> [Int] -> Int
lis n as_ = f (V.replicate n (-1)) as_
  where
    f :: Vector Int -> Int -> Int
    f dp 0 = undefined




