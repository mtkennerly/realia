#[test]
fn env_exists() {
    #[realia::env("REALIA_EMPTY")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(env("REALIA_UNDEFINED"))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn env_equals() {
    #[realia::env("REALIA_FULL", "1")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(env("REALIA_FULL", "2"))]
    fn negative1() -> bool {
        true
    }
    assert!(negative1());

    #[realia::not(env("REALIA_UNDEFINED", "1"))]
    fn negative2() -> bool {
        true
    }
    assert!(negative2());
}

#[test]
fn cmd_exists() {
    #[realia::cmd("cargo")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(cmd("."))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn any() {
    #[realia::any(env("REALIA_EMPTY"), env("REALIA_UNDEFINED"))]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(any(env("REALIA_UNDEFINED"), env("REALIA_UNDEFINED_2")))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn all() {
    #[realia::all(env("REALIA_EMPTY"), env("REALIA_FULL"))]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(all(env("REALIA_EMPTY"), env("REALIA_UNDEFINED")))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn attr() {
    #[realia::attr(env("REALIA_EMPTY"), derive(Debug))]
    struct Empty;
    format!("{:?}", Empty {});
}

#[test]
fn attr_const() {
    #[realia::attr(env("REALIA_EMPTY"), const)]
    fn foo() -> bool {
        true
    }
    #[allow(dead_code)]
    const POSITIVE: bool = foo();
}
