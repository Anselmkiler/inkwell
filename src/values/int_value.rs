use llvm_sys::core::{LLVMConstNot, LLVMConstNeg, LLVMConstNSWNeg, LLVMConstNUWNeg, LLVMConstAdd, LLVMConstNSWAdd, LLVMConstNUWAdd, LLVMConstSub, LLVMConstNSWSub, LLVMConstNUWSub, LLVMConstMul, LLVMConstNSWMul, LLVMConstNUWMul, LLVMConstUDiv, LLVMConstSDiv, LLVMConstSRem, LLVMConstURem, LLVMConstIntCast, LLVMConstXor, LLVMConstOr, LLVMConstAnd, LLVMConstExactSDiv};
use llvm_sys::prelude::LLVMValueRef;

use std::ffi::CStr;

use types::{AsTypeRef, IntType};
use values::traits::AsValueRef;
use values::{InstructionValue, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct IntValue {
    int_value: Value,
}

impl IntValue {
    pub(crate) fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        IntValue {
            int_value: Value::new(value),
        }
    }

    pub fn get_name(&self) -> &CStr {
        self.int_value.get_name()
    }

    pub fn set_name(&self, name: &str) {
        self.int_value.set_name(name);
    }

    pub fn get_type(&self) -> IntType {
        IntType::new(self.int_value.get_type())
    }

    pub fn is_null(&self) -> bool {
        self.int_value.is_null()
    }

    pub fn is_undef(&self) -> bool {
        self.int_value.is_undef()
    }

    pub fn print_to_string(&self) -> &CStr {
        self.int_value.print_to_string()
    }

    pub fn print_to_stderr(&self) {
        self.int_value.print_to_stderr()
    }

    pub fn as_instruction(&self) -> Option<InstructionValue> {
        self.int_value.as_instruction()
    }

    pub fn const_not(&self) -> Self {
        let value = unsafe {
            LLVMConstNot(self.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_neg(&self) -> Self {
        let value = unsafe {
            LLVMConstNeg(self.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_nsw_neg(&self) -> Self {
        let value = unsafe {
            LLVMConstNSWNeg(self.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_nuw_neg(&self) -> Self {
        let value = unsafe {
            LLVMConstNUWNeg(self.as_value_ref())
        };

        IntValue::new(value)
    }

    // TODO: operator overloading to call this
    pub fn const_add(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstAdd(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_nsw_add(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstNSWAdd(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_nuw_add(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstNUWAdd(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    // TODO: operator overloading to call this
    pub fn const_sub(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstSub(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_nsw_sub(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstNSWSub(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_nuw_sub(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstNUWSub(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    // TODO: operator overloading to call this
    pub fn const_mul(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstMul(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_nsw_mul(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstNSWMul(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_nuw_mul(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstNUWMul(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_unsigned_div(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstUDiv(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_signed_div(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstSDiv(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_exact_signed_div(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstExactSDiv(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_unsigned_remainder(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstURem(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_signed_remainder(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstSRem(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_and(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstAnd(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_or(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstOr(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    pub fn const_xor(&self, rhs: &IntValue) -> Self {
        let value = unsafe {
            LLVMConstXor(self.as_value_ref(), rhs.as_value_ref())
        };

        IntValue::new(value)
    }

    // TODO: Could infer is_signed from type (one day)
    pub fn const_cast(&self, int_type: &IntType, is_signed: bool) -> Self {
        let value = unsafe {
            LLVMConstIntCast(self.as_value_ref(), int_type.as_type_ref(), is_signed as i32)
        };

        IntValue::new(value)
    }
}

impl AsValueRef for IntValue {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.int_value.value
    }
}
