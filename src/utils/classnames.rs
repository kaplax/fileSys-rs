#[macro_export]
macro_rules! classnames {
    () => {
        String::new()
    };

    ($($token:tt),* $(,)?) => {{
        let mut classes = Vec::new();
        $(
            classnames!(@process &mut classes, $token);
        )*
        classes.join(" ")
    }};

    // 支持条件元组 `(class_name, condition)`
    (@process $classes:expr, ($class:expr, $cond:expr)) => {
        if $cond && !$class.is_empty() {
            $classes.push($class);
        }
    };

    // 支持简单字符串
    (@process $classes:expr, $class:expr) => {
        if !$class.is_empty() {
            $classes.push($class);
        }
    };


}
