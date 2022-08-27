use cc::Build;

fn main() {
    Build::new()
        .setup_compiler()
        .setup_openmp()
        .setup_sources()
        .compile("libsais.a");
}

enum ToolType {
    ClangLike,
    GnuLike,
    MsvcLike,
    Other,
}

trait BuildExtend {
    fn setup_compiler(&mut self) -> &mut Self;
    fn setup_openmp(&mut self) -> &mut Self;
    fn setup_sources(&mut self) -> &mut Self;
    fn tool_type(&self) -> ToolType;
}

impl BuildExtend for Build {
    fn setup_compiler(&mut self) -> &mut Self {
        if !is_debug() {
            match self.tool_type() {
                ToolType::ClangLike => self.opt_level_str("fast"),
                ToolType::GnuLike => self.opt_level(2),
                ToolType::MsvcLike => self.opt_level(2),
                _ => panic!("failed to configure compiler"),
            };
            self.define("NDEBUG", None);
        }
        self
    }

    fn setup_openmp(&mut self) -> &mut Self {
        if !cfg!(feature = "openmp") {
            return self;
        }
        std::env::var("DEP_OPENMP_FLAG").unwrap().split(" ").for_each(|f| { self.flag(f); });
        self
    }

    fn setup_sources(&mut self) -> &mut Self {
        let mut any_source = false;
        if cfg!(feature = "sais16") {
            self.file("libsais/src/libsais16.c");
            any_source = true;
        }
        if cfg!(feature = "sais32") {
            self.file("libsais/src/libsais.c");
            any_source = true;
        }
        if cfg!(feature = "sais64") {
            self.file("libsais/src/libsais64.c");
            any_source = true;
        }
        if !any_source {
            panic!("no libsais source files included");
        }
        self
    }

    fn tool_type(&self) -> ToolType {
        let tool = self.get_compiler();
        if tool.is_like_clang() {
            ToolType::ClangLike
        } else if tool.is_like_gnu() {
            ToolType::GnuLike
        } else if tool.is_like_msvc() {
            ToolType::MsvcLike
        } else {
            ToolType::Other
        }
    }
}

fn is_debug() -> bool {
    let profile = std::env::var("PROFILE").unwrap_or_default();
    profile == "debug"
}
