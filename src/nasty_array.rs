#[macro_export]
macro_rules! array {
    ($v: expr; 1) => {
        [$v]
    };
    ($v: expr; 2) => {
        [$v, $v]
    };
    ($v: expr; 3) => {
        [$v, $v, $v]
    };
    ($v: expr; 4) => {
        [$v, $v, $v, $v]
    };
    ($v: expr; 5) => {
        [$v, $v, $v, $v, $v]
    };
    ($v: expr; 6) => {
        [$v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 7) => {
        [$v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 8) => {
        [$v, $v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 9) => {
        [$v, $v, $v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 10) => {
        [$v, $v, $v, $v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 11) => {
        [$v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 12) => {
        [$v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 13) => {
        [$v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 14) => {
        [$v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 15) => {
        [$v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v]
    };
    ($v: expr; 16) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 17) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 18) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 19) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 20) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 21) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 22) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 23) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v,
        ]
    };
    ($v: expr; 24) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v,
        ]
    };
    ($v: expr; 25) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v, $v,
        ]
    };
    ($v: expr; 26) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v, $v, $v,
        ]
    };
    ($v: expr; 27) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 28) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 29) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 30) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 31) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
    ($v: expr; 32) => {
        [
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
            $v, $v, $v, $v, $v, $v, $v, $v, $v, $v,
        ]
    };
}
