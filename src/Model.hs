module Model where
data ContestItem = ContestItem {
  contest_start_time:: Integer,
  contest_end_time:: Integer,
  contest_source:: String,
  contest_name:: String,
  contest_link:: String
}