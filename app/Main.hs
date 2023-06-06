import Application () -- for YesodDispatch instance
import Foundation
import Yesod.Core
import Opts

main :: IO ()
main = warp 3000 App
