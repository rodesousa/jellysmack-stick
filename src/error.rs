use std::fmt;
use std::process;

pub enum StickError {
    InputError,
    FormatError,
    DuplicatePairError,
    DoublePipeError,
    InitError,
    ConstraintError,
    LengthError,
    IllegalError,
    WidthMissMatchError(usize, i32),
}

impl fmt::Display for StickError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StickError::InputError => write!(f, "An error occured during reading"),
            StickError::DuplicatePairError => write!(f, "Some head/tail aren't unique"),
            StickError::FormatError => write!(f, "There is a format error in the grid"),
            StickError::InitError => write!(f, "There is a error with width or height"),
            StickError::WidthMissMatchError(a, b) => {
                write!(f, "{} != {}, The size is mismatched", a, b)
            }

            StickError::ConstraintError => write!(
                f,
                "The constraints are not respected (width > 3 && height <= 100)"
            ),
            StickError::LengthError => write!(f, "The len of head and tail are different"),
            StickError::DoublePipeError => write!(f, "There are pipe side by side"),
            StickError::IllegalError => {
                write!(f, "Charater only accepted in the grid: '|', ' ' or '-'")
            }
        }
    }
}

// uniq function to exit
pub fn exit(error: StickError) {
    eprintln!("{}", error);
    process::exit(1);
}
