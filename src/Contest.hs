{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE QuasiQuotes       #-}
module Contest where

import Foundation
import Yesod.Core
    ( object, selectRep, provideJson, (.=), TypedContent )

-- getContestJsonR:: Maybe String -> Maybe Integer -> Handler TypedContent
-- getContestJsonR source day = selectRep $ do
getContestJsonR:: Handler TypedContent
getContestJsonR = selectRep $ do
  provideJson $ object ["msg" .= show 0]
