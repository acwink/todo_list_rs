use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub struct ParseTaskError(String);

impl fmt::Display for ParseTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ParseIntError> for ParseTaskError {
    fn from(_: ParseIntError) -> Self {
        ParseTaskError(format!("no is invalid digit"))
    }
}

#[derive(Clone)]
pub struct Task {
  pub no: u32,
  pub desc: String,
  pub status: TaskStatus,
}

impl fmt::Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:<20}{:<30}{:<20}", self.no, self.desc, self.status)
  }
}

impl FromStr for Task {
  type Err = ParseTaskError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
      let args: Vec<_> = s.split(',').collect();
      match args.len() {
          3 => {
              let no: u32 = args[0].parse()?;
              let desc = args[1].to_string();
              let status: TaskStatus = args[2].into();
              Ok(Self {
                  no,
                  desc,
                  status,
              })
          },
          _ => Err(ParseTaskError("argument's number is error".to_string())),
      }
  }
}

#[derive(PartialEq, Clone, Copy)]
pub enum TaskStatus {
  Pending,
  Processing,
  Completed,
}

impl From<&str> for TaskStatus {
  fn from(value: &str) -> Self {
      match value {
          "0" => Self::Pending,
          "1" => Self::Processing,
          "2" => Self::Completed,
          _ => Self::Pending,
      }
  }
}

impl From<TaskStatus> for &str {
  fn from(value: TaskStatus) -> Self {
      match value {
          TaskStatus::Pending => "0",
          TaskStatus::Processing => "1",
          TaskStatus::Completed => "2",
      }
  }
}

impl fmt::Display for TaskStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", match self {
          Self::Pending => "Pending",
          Self::Processing => "Processing",
          Self::Completed => "Completed",
      })
  }
}