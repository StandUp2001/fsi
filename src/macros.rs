#![macro_use]

macro_rules! my_matches {
    ($value:expr, $($arg:expr),*) => {
        {
            let lower = $value.to_lowercase();
            let a = lower.as_str();
            $(a == $arg.to_lowercase().as_str())||*
        }
    };
}

macro_rules! exit {
    ($code:expr, $( $msg:expr ),*) => {{
        println!($($msg),*);
        std::process::exit($code);
    }};
}

macro_rules! command {
    ($command:expr, $($arg:expr),*) => {
        std::process::Command::new($command)
            $(.arg($arg))*
            .output()
            .expect("Failed to execute command")
    };
}
