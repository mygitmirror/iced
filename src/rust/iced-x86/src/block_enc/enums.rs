/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use core::fmt;

// GENERATOR-BEGIN: RelocKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Relocation kind
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
pub enum RelocKind {
	/// 64-bit offset. Only used if it's 64-bit code.
	Offset64,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_RELOC_KIND: [&str; 1] = [
	"Offset64",
];
impl fmt::Debug for RelocKind {
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_RELOC_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for RelocKind {
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(not(feature = "javascript"), inline)]
	fn default() -> Self {
		RelocKind::Offset64
	}
}
// GENERATOR-END: RelocKind

// GENERATOR-BEGIN: BlockEncoderOptions
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Encoder options
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct BlockEncoderOptions;
impl BlockEncoderOptions {
	/// No option is set
	pub const NONE: u32 = 0x0000_0000;
	/// By default, branches get updated if the target is too far away, eg. `Jcc SHORT` -> `Jcc NEAR` or if 64-bit mode, `Jcc + JMP [RIP+mem]`. If this option is enabled, no branches are fixed.
	pub const DONT_FIX_BRANCHES: u32 = 0x0000_0001;
	/// The [`BlockEncoder`] should return [`RelocInfo`]s
	///
	/// [`BlockEncoder`]: struct.BlockEncoder.html
	/// [`RelocInfo`]: struct.RelocInfo.html
	pub const RETURN_RELOC_INFOS: u32 = 0x0000_0002;
	/// The [`BlockEncoder`] should return new instruction offsets
	///
	/// [`BlockEncoder`]: struct.BlockEncoder.html
	pub const RETURN_NEW_INSTRUCTION_OFFSETS: u32 = 0x0000_0004;
	/// The [`BlockEncoder`] should return [`ConstantOffsets`]
	///
	/// [`BlockEncoder`]: struct.BlockEncoder.html
	/// [`ConstantOffsets`]: struct.ConstantOffsets.html
	pub const RETURN_CONSTANT_OFFSETS: u32 = 0x0000_0008;
}
// GENERATOR-END: BlockEncoderOptions
