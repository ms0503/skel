use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkeletonNotFoundError;

impl Display for SkeletonNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Skeleton not found")
    }
}

impl std::error::Error for SkeletonNotFoundError {
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkeletonsDirNotFoundError;

impl Display for SkeletonsDirNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Skeletons directory not found")
    }
}

impl std::error::Error for SkeletonsDirNotFoundError {
}
