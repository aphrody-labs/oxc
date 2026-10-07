#![cfg(feature = "full")]

use std::path::Path;

use oxc::{
    CompilerInterface, codegen::CodegenReturn, diagnostics::Diagnostics, mangler::MangleOptions,
    minifier::CompressOptions, span::SourceType, transformer::TransformOptions,
};

struct CommentCompiler {
    transform: Option<TransformOptions>,
    compress: bool,
    mangle: bool,
    output: String,
}

impl CompilerInterface for CommentCompiler {
    fn transform_options(&self) -> Option<&TransformOptions> {
        self.transform.as_ref()
    }

    fn compress_options(&self) -> Option<CompressOptions> {
        self.compress.then(CompressOptions::default)
    }

    fn mangle_options(&self) -> Option<MangleOptions> {
        self.mangle.then(MangleOptions::default)
    }

    fn handle_errors(&mut self, errors: Diagnostics) {
        assert!(errors.is_empty(), "{errors:?}");
    }

    fn after_codegen(&mut self, result: CodegenReturn<'_>) {
        self.output = result.code;
    }
}

#[test]
fn compiler_pipeline_keeps_comment_owners_through_rebuilds() {
    for transform in [false, true] {
        for compress in [false, true] {
            let mut compiler = CommentCompiler {
                transform: transform.then(TransformOptions::default),
                compress,
                mangle: true,
                output: String::new(),
            };
            let source = "/* first */ if (false) gone();\n/* live */ consume(/* argument */ 1);";
            compiler.compile(source, SourceType::mjs(), Path::new("input.js"));
            assert!(compiler.output.contains("/* live */"), "{}", compiler.output);
            assert!(compiler.output.contains("/* argument */"), "{}", compiler.output);
            assert!(
                compiler.output.find("/* live */").unwrap()
                    < compiler.output.find("consume(").unwrap(),
                "{}",
                compiler.output,
            );
            assert_eq!(compiler.output.matches("/* first */").count(), 1);
        }
    }
}
