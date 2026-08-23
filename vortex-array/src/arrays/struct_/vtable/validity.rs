// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright the Vortex contributors

use vortex_error::VortexResult;

use crate::ExecutionCtx;
use crate::array::ArrayView;
use crate::array::ValidityVTable;
use crate::arrays::struct_::StructArrayExt;
use crate::arrays::struct_::vtable::Struct;
use crate::validity::Validity;

impl ValidityVTable<Struct> for Struct {
    fn validity(array: ArrayView<'_, Struct>, _ctx: &mut ExecutionCtx) -> VortexResult<Validity> {
        Ok(array.struct_validity())
    }
}
