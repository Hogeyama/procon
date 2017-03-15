
import Data.Set (Set)
import qualified Data.Set as S

main :: IO ()
main = do
  xs <- map readInt . words <$> getLine
  print $ solve xs

type Id = Int -- これを付けないと同じ長さの板があった時に死ぬ

solve :: [Int] -> Int
solve xs_ = f 0 $ S.fromList $ zip xs_ [0..]
  where
    f :: Int -> Set (Int,Id) -> Int
    f acc xs = case S.size xs of
      0 -> error "Impossible"
      1 -> acc
      _ -> let (min1@(n,_),xs1)  = S.deleteFindMin xs
               (min2@(m,_),xs2)  = S.deleteFindMin xs1
               xs3               = S.insert (merge min1 min2) xs2
           in  f (n+m+acc) xs3
    merge (n,id1) (m,_) = (n+m,id1)

readInt :: String -> Int
readInt = read

