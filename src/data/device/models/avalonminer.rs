use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AvalonMinerModel {
    A721,
    A741,
    A761,
    A821,
    A841,
    A851,
    A921,
    A1026,
    A1047,
    A1066,
    A1126,
    A1166,
    A1246,
    A1566,
    Nano3,
    Nano3S,
}
