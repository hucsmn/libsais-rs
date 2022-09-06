use cc::Build;
use target_tuples::{Architecture, OS, Target};

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
    fn target_arch(&self) -> Architecture;
    fn target_os(&self) -> OS;
}

impl BuildExtend for Build {
    fn setup_compiler(&mut self) -> &mut Self {
        if is_debug() {
            return self;
        }

        match self.tool_type() {
            ToolType::ClangLike => self.opt_level_str("fast"),
            ToolType::GnuLike => self.opt_level(2),
            ToolType::MsvcLike => self.opt_level(2),
            _ => panic!("failed to configure compiler"),
        };

        match (self.target_arch(), self.tool_type()) {
            (Architecture::X86_64, ToolType::ClangLike | ToolType::GnuLike) => self.flag("-march=skylake"),
            (Architecture::X86_64, ToolType::MsvcLike) => self.flag("/arch:AVX2"),
            _ => self,
        };

        self.define("NDEBUG", None);
        self
    }

    fn setup_openmp(&mut self) -> &mut Self {
        if !cfg!(feature = "openmp") {
            return self;
        }
        match self.tool_type() {
            ToolType::ClangLike => {
                self.flag("-fopenmp");
            }
            ToolType::GnuLike => {
                self.flag("-fopenmp");
                if self.target_os() == OS::Win32 {
                    // openmp-sys reports missing gomp.dll on *-pc-windows-gnu/mingw-w64, workaround for this case
                    println!("cargo:rustc-link-arg=-l:libgomp.dll.a");
                }
            }
            ToolType::MsvcLike => {
                self.flag("/openmp");
            }
            _ => panic!("failed to configure openmp for unsupported compiler"),
        };
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

    fn target_arch(&self) -> Architecture {
        environ("TARGET")
            .parse::<Target>()
            .map(|target| target.arch())
            .unwrap_or_else(|_| Architecture::Unknown)
    }

    fn target_os(&self) -> OS {
        environ("TARGET")
            .parse::<Target>()
            .ok()
            .and_then(|target| target.operating_system())
            .unwrap_or_else(|| OS::Unknown)
    }
}

fn is_debug() -> bool {
    environ("PROFILE") == "debug"
}

fn environ(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}
