import Application ()
-- for YesodDispatch instance
import Foundation
import Options.Applicative (execParser, fullDesc, info)
import Opts
import Yesod.Core

exec :: CWTask -> IO ()
exec (Serve port) = putStrLn ("Server starting on port " ++ show port) >> warp port App
exec _ = undefined

main :: IO ()
main = exec =<< execParser opts
  where
    opts = info cwOpts fullDesc
