use realia;

#[test]
fn crate_available() {
    #[realia::crate_available("syn")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(crate_available("!"))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn crate_equals() {
    #[realia::crate_equals("realia", "0.1.0")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(crate_equals("realia", "0.0.0"))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn crate_since() {
    #[realia::crate_since("syn", "1.0.0")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(crate_since("syn", "999"))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn crate_before() {
    #[realia::crate_before("syn", "999")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(crate_before("syn", "0"))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn crate_from_registry() {
    #[realia::crate_from_registry("syn")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(crate_from_registry("realia"))]
    fn negative() -> bool {
        true
    }
    assert!(negative());
}

#[test]
fn env() {
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
    #[realia::env_equals("REALIA_FULL", "1")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(env_equals("REALIA_FULL", "2"))]
    fn negative1() -> bool {
        true
    }
    assert!(negative1());

    #[realia::not(env_equals("REALIA_UNDEFINED", "1"))]
    fn negative2() -> bool {
        true
    }
    assert!(negative2());
}

#[test]
fn command() {
    #[realia::command("cargo")]
    fn positive() -> bool {
        true
    }
    assert!(positive());

    #[realia::not(command("."))]
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
    const positive: bool = foo();
    assert!(positive);
}
