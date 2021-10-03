use super::Mutator;
use crate::{ModuleInfo, Result, WasmMutate};
use rand::prelude::SmallRng;
use rand::{Rng, RngCore};
use wasm_encoder::{CodeSection, Export, ExportSection, Function, Instruction, Module};
use wasmparser::{CodeSectionReader, ExportSectionReader};

/// RenameExportMutator generates a random renaming of prexisting exports.
/// The export entry is selected randmonly and then a new `field` name is generated/
pub struct RenameExportMutator {
    /// The maximum length of the generated export entry
    pub max_name_size: u32,
}

impl RenameExportMutator {
    // Copied and transformed from wasm-smith name generation
    fn limited_string(&self, rnd: &mut SmallRng, info: &ModuleInfo, max_name_size: u32) -> String {
        let size = rnd.gen_range(1, max_name_size);
        let size = std::cmp::min(size, max_name_size);
        let mut str = vec![0u8; size as usize];
        rnd.fill_bytes(&mut str);

        let mut s = match std::str::from_utf8(&str) {
            Ok(s) => String::from(s),
            Err(e) => {
                let i = e.valid_up_to();
                let valid = &str[0..i];
                let s = unsafe {
                    debug_assert!(std::str::from_utf8(valid).is_ok());
                    std::str::from_utf8_unchecked(valid)
                };
                String::from(s)
            }
        };
        // Add one symbol at a time until it is not contained in the export field names
        while info.export_names.contains(&s.as_str()) {
            s.push_str(&self.limited_string(rnd, info, 2));
        }
        s
    }
}

impl Mutator for RenameExportMutator {
    fn mutate(
        &self,
        config: &WasmMutate,
        rnd: &mut SmallRng,
        info: &mut ModuleInfo,
    ) -> Result<Module> {
        let mut exports = ExportSection::new();
        let mut reader = ExportSectionReader::new(info.get_exports_section().data, 0)?;
        let max_exports = reader.get_count() as u64;
        let skip_at = rnd.gen_range(0, max_exports);

        (0..max_exports).for_each(|i| {
            let export = reader.read().unwrap();

            let new_name = if skip_at != i {
                // otherwise bypass
                String::from(export.field)
            } else {
                let new_name = self.limited_string(rnd, info, self.max_name_size);
                log::debug!("Renaming export {:?} by {:?}", export, new_name);
                new_name
            };

            match export.kind {
                wasmparser::ExternalKind::Function => {
                    exports.export(new_name.as_str(), Export::Function(export.index));
                }
                wasmparser::ExternalKind::Table => {
                    exports.export(new_name.as_str(), Export::Table(export.index));
                }
                wasmparser::ExternalKind::Memory => {
                    exports.export(new_name.as_str(), Export::Memory(export.index));
                }
                wasmparser::ExternalKind::Global => {
                    exports.export(new_name.as_str(), Export::Global(export.index));
                }
                wasmparser::ExternalKind::Module => {
                    exports.export(new_name.as_str(), Export::Module(export.index));
                }
                wasmparser::ExternalKind::Instance => {
                    exports.export(new_name.as_str(), Export::Instance(export.index));
                }
                _ => {
                    panic!("Unknown export {:?}", export)
                }
            }
        });
        Ok(info.replace_section(info.exports.unwrap(), &exports))
    }

    fn can_mutate<'a>(&self, _: &'a WasmMutate, info: &ModuleInfo) -> Result<bool> {
        Ok(info.has_exports() && info.exports_count > 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::WasmMutate;
    use rand::{rngs::SmallRng, SeedableRng};

    use super::{Mutator, RenameExportMutator};

    #[test]
    fn test_rename_export_mutator() {
        // From https://developer.mozilla.org/en-US/docs/WebAssembly/Text_format_to_wasm

        crate::mutators::match_mutation(
            r#"
        (module
            (func (export "exported_func") (result i32)
                i32.const 42
            )
        )
        "#,
            &RenameExportMutator { max_name_size: 2 }, // the string is empty,
            r#"(module
            (type (;0;) (func (result i32)))
            (func (;0;) (type 0) (result i32)
            i32.const 42)
        (export "" (func 0)))"#,
        );
    }
}
