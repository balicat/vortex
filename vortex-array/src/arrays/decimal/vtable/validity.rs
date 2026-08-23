// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright the Vortex contributors

use vortex_error::VortexResult;

use crate::ExecutionCtx;
use crate::array::ArrayView;
use crate::array::ValidityVTable;
use crate::arrays::decimal::DecimalArrayExt;
use crate::arrays::decimal::vtable::Decimal;
use crate::validity::Validity;

impl ValidityVTable<Decimal> for Decimal {
    fn validity(array: ArrayView<'_, Decimal>, _ctx: &mut ExecutionCtx) -> VortexResult<Validity> {
        Ok(DecimalArrayExt::validity(&array))
    }
}
