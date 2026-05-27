use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum FunctionRuntime {
    #[serde(rename = "node-14.5")]
    #[default]
    Node145,
    #[serde(rename = "node-16.0")]
    Node160,
    #[serde(rename = "node-18.0")]
    Node180,
    #[serde(rename = "node-19.0")]
    Node190,
    #[serde(rename = "node-20.0")]
    Node200,
    #[serde(rename = "node-21.0")]
    Node210,
    #[serde(rename = "node-22")]
    Node22,
    #[serde(rename = "node-23")]
    Node23,
    #[serde(rename = "node-24")]
    Node24,
    #[serde(rename = "node-25")]
    Node25,
    #[serde(rename = "php-8.0")]
    Php80,
    #[serde(rename = "php-8.1")]
    Php81,
    #[serde(rename = "php-8.2")]
    Php82,
    #[serde(rename = "php-8.3")]
    Php83,
    #[serde(rename = "php-8.4")]
    Php84,
    #[serde(rename = "ruby-3.0")]
    Ruby30,
    #[serde(rename = "ruby-3.1")]
    Ruby31,
    #[serde(rename = "ruby-3.2")]
    Ruby32,
    #[serde(rename = "ruby-3.3")]
    Ruby33,
    #[serde(rename = "ruby-3.4")]
    Ruby34,
    #[serde(rename = "ruby-4.0")]
    Ruby40,
    #[serde(rename = "python-3.8")]
    Python38,
    #[serde(rename = "python-3.9")]
    Python39,
    #[serde(rename = "python-3.10")]
    Python310,
    #[serde(rename = "python-3.11")]
    Python311,
    #[serde(rename = "python-3.12")]
    Python312,
    #[serde(rename = "python-3.13")]
    Python313,
    #[serde(rename = "python-3.14")]
    Python314,
    #[serde(rename = "python-ml-3.11")]
    PythonMl311,
    #[serde(rename = "python-ml-3.12")]
    PythonMl312,
    #[serde(rename = "python-ml-3.13")]
    PythonMl313,
    #[serde(rename = "deno-1.21")]
    Deno121,
    #[serde(rename = "deno-1.24")]
    Deno124,
    #[serde(rename = "deno-1.35")]
    Deno135,
    #[serde(rename = "deno-1.40")]
    Deno140,
    #[serde(rename = "deno-1.46")]
    Deno146,
    #[serde(rename = "deno-2.0")]
    Deno20,
    #[serde(rename = "deno-2.5")]
    Deno25,
    #[serde(rename = "deno-2.6")]
    Deno26,
    #[serde(rename = "dart-2.15")]
    Dart215,
    #[serde(rename = "dart-2.16")]
    Dart216,
    #[serde(rename = "dart-2.17")]
    Dart217,
    #[serde(rename = "dart-2.18")]
    Dart218,
    #[serde(rename = "dart-2.19")]
    Dart219,
    #[serde(rename = "dart-3.0")]
    Dart30,
    #[serde(rename = "dart-3.1")]
    Dart31,
    #[serde(rename = "dart-3.3")]
    Dart33,
    #[serde(rename = "dart-3.5")]
    Dart35,
    #[serde(rename = "dart-3.8")]
    Dart38,
    #[serde(rename = "dart-3.9")]
    Dart39,
    #[serde(rename = "dart-3.10")]
    Dart310,
    #[serde(rename = "dart-3.11")]
    Dart311,
    #[serde(rename = "dart-3.12")]
    Dart312,
    #[serde(rename = "dotnet-6.0")]
    Dotnet60,
    #[serde(rename = "dotnet-7.0")]
    Dotnet70,
    #[serde(rename = "dotnet-8.0")]
    Dotnet80,
    #[serde(rename = "dotnet-10")]
    Dotnet10,
    #[serde(rename = "java-8.0")]
    Java80,
    #[serde(rename = "java-11.0")]
    Java110,
    #[serde(rename = "java-17.0")]
    Java170,
    #[serde(rename = "java-18.0")]
    Java180,
    #[serde(rename = "java-21.0")]
    Java210,
    #[serde(rename = "java-22")]
    Java22,
    #[serde(rename = "java-25")]
    Java25,
    #[serde(rename = "swift-5.5")]
    Swift55,
    #[serde(rename = "swift-5.8")]
    Swift58,
    #[serde(rename = "swift-5.9")]
    Swift59,
    #[serde(rename = "swift-5.10")]
    Swift510,
    #[serde(rename = "swift-6.2")]
    Swift62,
    #[serde(rename = "kotlin-1.6")]
    Kotlin16,
    #[serde(rename = "kotlin-1.8")]
    Kotlin18,
    #[serde(rename = "kotlin-1.9")]
    Kotlin19,
    #[serde(rename = "kotlin-2.0")]
    Kotlin20,
    #[serde(rename = "kotlin-2.3")]
    Kotlin23,
    #[serde(rename = "cpp-17")]
    Cpp17,
    #[serde(rename = "cpp-20")]
    Cpp20,
    #[serde(rename = "bun-1.0")]
    Bun10,
    #[serde(rename = "bun-1.1")]
    Bun11,
    #[serde(rename = "bun-1.2")]
    Bun12,
    #[serde(rename = "bun-1.3")]
    Bun13,
    #[serde(rename = "go-1.23")]
    Go123,
    #[serde(rename = "go-1.24")]
    Go124,
    #[serde(rename = "go-1.25")]
    Go125,
    #[serde(rename = "go-1.26")]
    Go126,
    #[serde(rename = "rust-1.83")]
    Rust183,
    #[serde(rename = "static-1")]
    Static1,
    #[serde(rename = "flutter-3.24")]
    Flutter324,
    #[serde(rename = "flutter-3.27")]
    Flutter327,
    #[serde(rename = "flutter-3.29")]
    Flutter329,
    #[serde(rename = "flutter-3.32")]
    Flutter332,
    #[serde(rename = "flutter-3.35")]
    Flutter335,
    #[serde(rename = "flutter-3.38")]
    Flutter338,
    #[serde(rename = "flutter-3.41")]
    Flutter341,
    #[serde(rename = "flutter-3.44")]
    Flutter344,
}

impl FunctionRuntime {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            FunctionRuntime::Node145 => "node-14.5",
            FunctionRuntime::Node160 => "node-16.0",
            FunctionRuntime::Node180 => "node-18.0",
            FunctionRuntime::Node190 => "node-19.0",
            FunctionRuntime::Node200 => "node-20.0",
            FunctionRuntime::Node210 => "node-21.0",
            FunctionRuntime::Node22 => "node-22",
            FunctionRuntime::Node23 => "node-23",
            FunctionRuntime::Node24 => "node-24",
            FunctionRuntime::Node25 => "node-25",
            FunctionRuntime::Php80 => "php-8.0",
            FunctionRuntime::Php81 => "php-8.1",
            FunctionRuntime::Php82 => "php-8.2",
            FunctionRuntime::Php83 => "php-8.3",
            FunctionRuntime::Php84 => "php-8.4",
            FunctionRuntime::Ruby30 => "ruby-3.0",
            FunctionRuntime::Ruby31 => "ruby-3.1",
            FunctionRuntime::Ruby32 => "ruby-3.2",
            FunctionRuntime::Ruby33 => "ruby-3.3",
            FunctionRuntime::Ruby34 => "ruby-3.4",
            FunctionRuntime::Ruby40 => "ruby-4.0",
            FunctionRuntime::Python38 => "python-3.8",
            FunctionRuntime::Python39 => "python-3.9",
            FunctionRuntime::Python310 => "python-3.10",
            FunctionRuntime::Python311 => "python-3.11",
            FunctionRuntime::Python312 => "python-3.12",
            FunctionRuntime::Python313 => "python-3.13",
            FunctionRuntime::Python314 => "python-3.14",
            FunctionRuntime::PythonMl311 => "python-ml-3.11",
            FunctionRuntime::PythonMl312 => "python-ml-3.12",
            FunctionRuntime::PythonMl313 => "python-ml-3.13",
            FunctionRuntime::Deno121 => "deno-1.21",
            FunctionRuntime::Deno124 => "deno-1.24",
            FunctionRuntime::Deno135 => "deno-1.35",
            FunctionRuntime::Deno140 => "deno-1.40",
            FunctionRuntime::Deno146 => "deno-1.46",
            FunctionRuntime::Deno20 => "deno-2.0",
            FunctionRuntime::Deno25 => "deno-2.5",
            FunctionRuntime::Deno26 => "deno-2.6",
            FunctionRuntime::Dart215 => "dart-2.15",
            FunctionRuntime::Dart216 => "dart-2.16",
            FunctionRuntime::Dart217 => "dart-2.17",
            FunctionRuntime::Dart218 => "dart-2.18",
            FunctionRuntime::Dart219 => "dart-2.19",
            FunctionRuntime::Dart30 => "dart-3.0",
            FunctionRuntime::Dart31 => "dart-3.1",
            FunctionRuntime::Dart33 => "dart-3.3",
            FunctionRuntime::Dart35 => "dart-3.5",
            FunctionRuntime::Dart38 => "dart-3.8",
            FunctionRuntime::Dart39 => "dart-3.9",
            FunctionRuntime::Dart310 => "dart-3.10",
            FunctionRuntime::Dart311 => "dart-3.11",
            FunctionRuntime::Dart312 => "dart-3.12",
            FunctionRuntime::Dotnet60 => "dotnet-6.0",
            FunctionRuntime::Dotnet70 => "dotnet-7.0",
            FunctionRuntime::Dotnet80 => "dotnet-8.0",
            FunctionRuntime::Dotnet10 => "dotnet-10",
            FunctionRuntime::Java80 => "java-8.0",
            FunctionRuntime::Java110 => "java-11.0",
            FunctionRuntime::Java170 => "java-17.0",
            FunctionRuntime::Java180 => "java-18.0",
            FunctionRuntime::Java210 => "java-21.0",
            FunctionRuntime::Java22 => "java-22",
            FunctionRuntime::Java25 => "java-25",
            FunctionRuntime::Swift55 => "swift-5.5",
            FunctionRuntime::Swift58 => "swift-5.8",
            FunctionRuntime::Swift59 => "swift-5.9",
            FunctionRuntime::Swift510 => "swift-5.10",
            FunctionRuntime::Swift62 => "swift-6.2",
            FunctionRuntime::Kotlin16 => "kotlin-1.6",
            FunctionRuntime::Kotlin18 => "kotlin-1.8",
            FunctionRuntime::Kotlin19 => "kotlin-1.9",
            FunctionRuntime::Kotlin20 => "kotlin-2.0",
            FunctionRuntime::Kotlin23 => "kotlin-2.3",
            FunctionRuntime::Cpp17 => "cpp-17",
            FunctionRuntime::Cpp20 => "cpp-20",
            FunctionRuntime::Bun10 => "bun-1.0",
            FunctionRuntime::Bun11 => "bun-1.1",
            FunctionRuntime::Bun12 => "bun-1.2",
            FunctionRuntime::Bun13 => "bun-1.3",
            FunctionRuntime::Go123 => "go-1.23",
            FunctionRuntime::Go124 => "go-1.24",
            FunctionRuntime::Go125 => "go-1.25",
            FunctionRuntime::Go126 => "go-1.26",
            FunctionRuntime::Rust183 => "rust-1.83",
            FunctionRuntime::Static1 => "static-1",
            FunctionRuntime::Flutter324 => "flutter-3.24",
            FunctionRuntime::Flutter327 => "flutter-3.27",
            FunctionRuntime::Flutter329 => "flutter-3.29",
            FunctionRuntime::Flutter332 => "flutter-3.32",
            FunctionRuntime::Flutter335 => "flutter-3.35",
            FunctionRuntime::Flutter338 => "flutter-3.38",
            FunctionRuntime::Flutter341 => "flutter-3.41",
            FunctionRuntime::Flutter344 => "flutter-3.44",
        }
    }
}

impl std::fmt::Display for FunctionRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
