// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use bytecode_verifier::RecursiveStructDefChecker;
use proptest::prelude::*;
use vm::{checks::BoundsChecker, file_format::CompiledModule};

proptest! {
    #[test]
    fn valid_recursive_struct_defs(module in CompiledModule::valid_strategy(20)) {
        prop_assert!(BoundsChecker::new(&module).verify().is_empty());
        let recursive_checker = RecursiveStructDefChecker::new(&module);
        prop_assert!(recursive_checker.verify().is_empty());
    }
}
